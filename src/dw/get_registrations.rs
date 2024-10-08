use std::sync::{Arc, RwLock};

use axum::extract::State;

use crate::AppState;

pub async fn get_registrations(State(state): State<Arc<RwLock<AppState>>>) -> String {
    println!("Getting all registration");

    let read = state.read().unwrap();
    serde_json::to_string(&read.registrations).unwrap()
}
