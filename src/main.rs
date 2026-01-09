use axum::{routing::get, Router};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod handlers;
mod models;
mod services;

use handlers::ws::AppState;
use models::Article;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create broadcast channel for WebSocket clients
    let (tx, _rx) = broadcast::channel::<Article>(100);
    let app_state = AppState { tx };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Routes that need state (WebSocket and broadcast)
    let stateful_routes = Router::new()
        .route("/ws", get(handlers::ws::ws_handler))
        .route("/api/news/broadcast", get(handlers::news::fetch_and_broadcast))
        .with_state(app_state);

    // Routes that don't need state
    let stateless_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/news", get(handlers::news::get_news))
        .route("/api/sources", get(handlers::news::get_sources));

    let app = Router::new()
        .merge(stateful_routes)
        .merge(stateless_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3004".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server running on http://localhost:{}", port);
    tracing::info!("WebSocket available at ws://localhost:{}/ws", port);
    axum::serve(listener, app).await.unwrap();
}
