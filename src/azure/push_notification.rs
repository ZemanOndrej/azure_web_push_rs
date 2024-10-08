use axum::http::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::azure::{ConnectionStringUtility, CONNECTION_STRING, HUB_NAME, NAMESPACE};

pub async fn push_notification() -> () {
    println!("Pushing notification");

    let client = Client::new();

    let connection_string = ConnectionStringUtility::new(CONNECTION_STRING);

    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages?api-version=2024-01",
        NAMESPACE, HUB_NAME
    );

    let authorization_token = connection_string.get_sas_token(&connection_string.endpoint, 10);
    let mut header_map = HeaderMap::new();
    header_map.insert(
        "Authorization",
        HeaderValue::from_str(&authorization_token).unwrap(),
    );
    header_map.insert("x-ms-version", HeaderValue::from_str("2024-01").unwrap());
    header_map.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    header_map.insert(
        "ServiceBusNotification-Format",
        HeaderValue::from_str("browser").unwrap(),
    );
    let body = r#"{"data": {"message": "Hello from Rust!"}}"#;
    let response = client
        .post(&url)
        .headers(header_map)
        .body(body)
        .send()
        .await
        .unwrap();

    let status = response.status();
    let res = response.text().await.unwrap();
    if status.is_success() {
        println!("Notification sent successfully!");
    } else {
        println!("Failed to send notification!");
    }
    println!("Response: {:?}", res);

    // push notification to azure hub
}
