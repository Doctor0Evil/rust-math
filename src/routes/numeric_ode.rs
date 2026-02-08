use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::math::ode::solve_simple_ode;

#[derive(Debug, Deserialize)]
pub struct OdeRequest {
    pub y0: Vec<f64>,
    pub t0: f64,
    pub t1: f64,
    pub dt: f64,
}

#[derive(Debug, Serialize)]
pub struct OdeResponse {
    pub points: Vec<(f64, Vec<f64>)>,
}

async fn solve_ode(Json(payload): Json<OdeRequest>) -> Result<Json<OdeResponse>, AppError> {
    let points = solve_simple_ode(payload.y0, payload.t0, payload.t1, payload.dt)?;
    Ok(Json(OdeResponse { points }))
}

pub fn router() -> Router {
    Router::new().route("/ode/solve", post(solve_ode))
}
