use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid input: {0}")]
    BadRequest(String),
    #[error("Math error: {0}")]
    MathError(String),
    #[error("Internal server error")]
    Internal,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: u16,
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::MathError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string()),
        };

        let body = ErrorBody {
            code: status.as_u16(),
            error: self.to_string(),
            message,
        };

        (status, Json(body)).into_response()
    }
}
