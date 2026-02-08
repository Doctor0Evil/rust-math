use symbolica::atom::Atom;

use crate::error::AppError;
use super::backend::{SymbolicBackend, symbolica_backend::SymbolicaBackend};
use super::parse::Expr;

pub fn simplify_expression(expr: &Expr) -> Result<Expr, AppError> {
    SymbolicaBackend::simplify(expr)
}
                                                     }
    let simplified = expr.simplify();
    Ok(simplified)
}
