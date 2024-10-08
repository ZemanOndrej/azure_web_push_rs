use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, RwLock},
};

use axum::{extract::State, Json};
use web_push::SubscriptionInfo;

use crate::{AppState, RegisterPayload};

pub async fn post_registration(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<RegisterPayload>,
) -> String {
    println!("Registering new client");
    let subscription =
        SubscriptionInfo::new(payload.endpoint, payload.keys.p256dh, payload.keys.auth);
    let mut write = state.write().unwrap();
    let mut hasher = DefaultHasher::new();
    subscription.hash(&mut hasher);
    let hash = hasher.finish();

    write.registrations.insert(hash, subscription);

    "Registration successful".to_string()
}
