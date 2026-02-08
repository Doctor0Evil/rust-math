use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SimplifyRequest {
    pub expression: String,
}

#[derive(Debug, Serialize)]
pub struct SimplifyResponse {
    pub input: String,
    pub simplified: String,
    pub latex: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DifferentiateRequest {
    pub expression: String,
    pub var: String,
    pub order: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct DifferentiateResponse {
    pub input: String,
    pub derivative: String,
    pub latex: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub expression: String,
    pub vars: Option<Vec<VarAssignment>>,
}

#[derive(Debug, Deserialize)]
pub struct VarAssignment {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct EvaluateResponse {
    pub input: String,
    pub value: Option<f64>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatrixSolveRequest {
    pub matrix: Vec<Vec<f64>>,
    pub rhs: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct MatrixSolveResponse {
    pub solution: Option<Vec<f64>>,
    pub error: Option<String>,
}
