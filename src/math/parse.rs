use crate::error::AppError;
use symbolica::atom::Atom;
use symbolica::parser::parse;

pub fn parse_expression(expr: &str) -> Result<Atom, AppError> {
    parse(expr).map_err(|e| AppError::BadRequest(format!("Parse error: {}", e)))
}
