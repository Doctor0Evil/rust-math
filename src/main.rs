use rust_math::telemetry;
use rust_math::config::AppConfig;
use rust_math::routes;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    telemetry::init_tracing();

    let config = AppConfig::from_env().expect("Failed to load configuration");

    let app = Router::new()
        .merge(routes::health_router())
        .nest("/v1", routes::api_router())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((config.server.host, config.server.port));
    tracing::info!("Starting rust-math server on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
