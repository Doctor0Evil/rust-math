use crate::error::AppError;
use symbolica::atom::Atom;
use std::collections::HashMap;

pub fn evaluate_expression(
    expr: &Atom,
    vars: &HashMap<String, f64>,
) -> Result<f64, AppError> {
    // Using Symbolica or a numeric evaluation mapping as supported by the crate.[web:3][web:7]
    let mut env = symbolica::eval::EvalEnv::new();
    for (name, val) in vars {
        env.set_var(name, *val);
    }
    expr.evaluate(&env)
        .map_err(|e| AppError::MathError(format!("Evaluation error: {}", e)))
}
