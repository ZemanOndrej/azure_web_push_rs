use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    routing::{get, post},
    Router,
};
use azure_web_push_rs::*;
use reqwest::Client;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let state = AppState {
        registrations: HashMap::new(),
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/registration", post(dw::post_registration))
        .route("/registrations", get(dw::get_registrations))
        .route("/notification", post(dw::post_notification))
        // Azure routes
        .route("/azure/registrations", get(azure::get_registrations))
        .route("/azure/register", post(azure::register))
        .route("/azure/notification", post(azure::push_notification))
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
        .with_state(Arc::from(RwLock::new(state)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Listening on: http://{}",
        listener.local_addr().unwrap().to_string().as_str()
    );
    axum::serve(listener, app).await.unwrap();
}
// basic handler that responds with a static string
async fn root() -> String {
    let client = Client::new();
    let url = format!("https://jsonplaceholder.typicode.com/todos/{}", 1);

    let response = client.get(&url).send().await.unwrap();
    let body = response.text().await.unwrap().to_string();
    println!("body: {}", body);

    body
}
