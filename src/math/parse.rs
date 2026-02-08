use symbolica::atom::Atom;
use symbolica::parser::parse;
use crate::error::AppError;
use super::backend::{SymbolicBackend, symbolica_backend::SymbolicaBackend};

pub type Expr = <SymbolicaBackend as SymbolicBackend>::Expr;

pub fn parse_expression(expr: &str) -> Result<Expr, AppError> {
    SymbolicaBackend::parse(expr)
}
