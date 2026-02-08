use crate::error::AppError;
use symbolica::atom::Atom;

pub fn simplify_expression(expr: &Atom) -> Result<Atom, AppError> {
    // Symbolica supports simplification on Atom expressions.[web:3][web:7]
    let simplified = expr.simplify();
    Ok(simplified)
}
