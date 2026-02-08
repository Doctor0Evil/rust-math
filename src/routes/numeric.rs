use axum::{routing::post, Json, Router};
use crate::models::api::{MatrixSolveRequest, MatrixSolveResponse};
use crate::math::solve_linear_system;

async fn solve_matrix(
    Json(payload): Json<MatrixSolveRequest>,
) -> Json<MatrixSolveResponse> {
    match solve_linear_system(payload.matrix, payload.rhs) {
        Ok(solution) => Json(MatrixSolveResponse {
            solution: Some(solution),
            error: None,
        }),
        Err(e) => Json(MatrixSolveResponse {
            solution: None,
            error: Some(e.to_string()),
        }),
    }
}

pub fn router() -> Router {
    Router::new().route("/matrix/solve", post(solve_matrix))
}
