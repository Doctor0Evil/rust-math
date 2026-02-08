use axum::{routing::post, Json, Router};
use crate::models::api::{
    SimplifyRequest, SimplifyResponse, DifferentiateRequest, DifferentiateResponse,
    EvaluateRequest, EvaluateResponse, VarAssignment,
};
use crate::math::{parse_expression, simplify_expression, differentiate_expression, evaluate_expression};
use crate::error::AppError;
use std::collections::HashMap;

async fn simplify(Json(payload): Json<SimplifyRequest>) -> Result<Json<SimplifyResponse>, AppError> {
    let expr = parse_expression(&payload.expression)?;
    let simplified = simplify_expression(&expr)?;
    let simplified_str = simplified.to_string();
    // If the crate supports LaTeX rendering you can add it here.[web:3][web:7]
    let resp = SimplifyResponse {
        input: payload.expression,
        simplified: simplified_str,
        latex: None,
    };
    Ok(Json(resp))
}

async fn differentiate(
    Json(payload): Json<DifferentiateRequest>,
) -> Result<Json<DifferentiateResponse>, AppError> {
    let expr = parse_expression(&payload.expression)?;
    let order = payload.order.unwrap_or(1);
    let deriv = differentiate_expression(&expr, &payload.var, order)?;
    let deriv_str = deriv.to_string();
    let resp = DifferentiateResponse {
        input: payload.expression,
        derivative: deriv_str,
        latex: None,
    };
    Ok(Json(resp))
}

async fn evaluate(
    Json(payload): Json<EvaluateRequest>,
) -> Result<Json<EvaluateResponse>, AppError> {
    let expr = parse_expression(&payload.expression)?;
    let mut vars_map = HashMap::new();
    if let Some(vars) = payload.vars {
        for VarAssignment { name, value } in vars {
            vars_map.insert(name, value);
        }
    }

    match evaluate_expression(&expr, &vars_map) {
        Ok(value) => Ok(Json(EvaluateResponse {
            input: payload.expression,
            value: Some(value),
            error: None,
        })),
        Err(e) => Ok(Json(EvaluateResponse {
            input: payload.expression,
            value: None,
            error: Some(e.to_string()),
        })),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/simplify", post(simplify))
        .route("/differentiate", post(differentiate))
        .route("/evaluate", post(evaluate))
}
