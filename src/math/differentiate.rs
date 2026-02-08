use crate::error::AppError;
use super::backend::{SymbolicBackend, symbolica_backend::SymbolicaBackend};
use super::parse::Expr;

pub fn differentiate_expression(
    expr: &Expr,
    var: &str,
    order: u8,
) -> Result<Expr, AppError> {
    if var.trim().is_empty() {
        return Err(AppError::BadRequest("missing differentiation variable".into()));
    }
    SymbolicaBackend::differentiate(expr, var, order)
}
