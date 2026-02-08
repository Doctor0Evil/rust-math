use axum::Router;

pub mod health;
pub mod symbolic;
pub mod numeric;
pub mod render;
pub mod meta;

pub fn health_router() -> Router {
    Router::new().merge(health::router())
}

pub fn api_router() -> Router {
    Router::new()
        .nest("/symbolic", symbolic::router())
        .nest("/numeric", numeric::router())
        .nest("/render", render::router())
        .nest("/meta", meta::router())
}
