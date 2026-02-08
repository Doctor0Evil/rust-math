use crate::error::AppError;
use symbolica::atom::Atom;

pub fn differentiate_expression(expr: &Atom, var: &str, order: u8) -> Result<Atom, AppError> {
    let mut result = expr.clone();
    for _ in 0..order {
        result = result
            .derivative(var)
            .map_err(|e| AppError::MathError(format!("Derivative error: {}", e)))?;
    }
    Ok(result)
}
