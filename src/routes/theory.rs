use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::theory::{logic::Prop, prover::prove_prop};

#[derive(Debug, Deserialize)]
pub struct ProveRequest {
    pub formula: String,
}

#[derive(Debug, Serialize)]
pub struct ProveResponse {
    pub success: bool,
    pub conclusion: String,
    pub steps: Vec<String>,
}

async fn prove(Json(_payload): Json<ProveRequest>) -> Json<ProveResponse> {
    // TODO: parse payload.formula → Prop
    let phi = Prop::Var("P".into());
    let proof = prove_prop(&phi);

    Json(ProveResponse {
        success: proof.success,
        conclusion: proof.conclusion,
        steps: proof.steps.into_iter().map(|s| s.formula).collect(),
    })
}

pub fn router() -> Router {
    Router::new().route("/prove", post(prove))
}
