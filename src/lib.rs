use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use web_push::SubscriptionInfo;

pub mod azure;
pub mod dw;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterPayload {
    endpoint: String,
    expiration_time: Option<i64>,
    keys: Keys,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keys {
    p256dh: String,
    auth: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub registrations: HashMap<u64, SubscriptionInfo>,
}
