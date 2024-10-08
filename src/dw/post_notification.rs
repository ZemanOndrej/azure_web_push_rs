use std::{
    fs::File,
    sync::{Arc, RwLock},
};

use crate::AppState;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::{json, Value};
use web_push::{
    ContentEncoding, IsahcWebPushClient, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

fn handle_error(e: web_push::WebPushError) -> Json<Value> {
    let result = Json(json!({"error": e.to_string()}));

    result
}

#[debug_handler]
pub async fn post_notification(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, Json<Value>> {
    println!("Pushing notification to all clients");

    let registrations = state.read().unwrap().registrations.clone();

    for (key, subscription_info) in registrations.iter() {
        println!("Key: {}, Value: {:?}", key, subscription_info);
        let file = File::open("private_key.pem").unwrap();

        let sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)
            .map_err(handle_error)?
            .build()
            .map_err(handle_error)?;

        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        let content = "Encrypted payload to be sent in the notification".as_bytes();
        builder.set_payload(ContentEncoding::Aes128Gcm, content);
        builder.set_vapid_signature(sig_builder);

        let client = IsahcWebPushClient::new().map_err(handle_error)?;

        //Finally, send the notification!
        client
            .send(builder.build().map_err(handle_error)?)
            .await
            .map_err(handle_error)?;
    }

    Ok(Json(json!({"message": "Notifications sent successfully"})))
}
