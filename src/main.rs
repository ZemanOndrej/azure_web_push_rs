use axum::{
    routing::{get, post},
    Router,
};
use azure_web_push_rs::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tower_http::services::ServeDir;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeFile,
};

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
    let api_router = Router::new()
        .route("/registration", post(dw::post_registration))
        .route("/registrations", get(dw::get_registrations))
        .route("/notification", post(dw::post_notification))
        // Azure routes
        .route("/azure/registrations", get(azure::get_registrations))
        .route("/azure/register", post(azure::register))
        .route("/azure/notification", post(azure::push_notification));

    // build our application with a route
    let app = Router::new()
        .nest("/api", api_router)
        .nest_service("/", ServeFile::new("public/index.html"))
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
