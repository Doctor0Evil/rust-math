use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Ast {
    pub repr: String,
}

pub trait SymbolicBackend {
    type Expr: Clone;

    fn parse(expr: &str) -> Result<Self::Expr, AppError>;
    fn simplify(expr: &Self::Expr) -> Result<Self::Expr, AppError>;
    fn differentiate(expr: &Self::Expr, var: &str, order: u8) -> Result<Self::Expr, AppError>;
    fn to_string(expr: &Self::Expr) -> String;
    fn to_latex(expr: &Self::Expr) -> Option<String>;
}

#[cfg(feature = "symbolica-backend")]
pub mod symbolica_backend {
    use super::SymbolicBackend;
    use crate::error::AppError;
    use symbolica::atom::Atom;
    use symbolica::parser::parse;

    pub struct SymbolicaBackend;

    impl SymbolicBackend for SymbolicaBackend {
        type Expr = Atom;

        fn parse(expr: &str) -> Result<Self::Expr, AppError> {
            parse(expr).map_err(|e| AppError::BadRequest(format!("parse error: {}", e)))
        }

        fn simplify(expr: &Self::Expr) -> Result<Self::Expr, AppError> {
            Ok(expr.simplify()) // Symbolica provides simplify()[web:3][web:33]
        }

        fn differentiate(expr: &Self::Expr, var: &str, order: u8) -> Result<Self::Expr, AppError> {
            let mut res = expr.clone();
            for _ in 0..order {
                res = res
                    .derivative(var)
                    .map_err(|e| AppError::MathError(format!("derivative error: {}", e)))?;
            }
            Ok(res)
        }

        fn to_string(expr: &Self::Expr) -> String {
            expr.to_string()
        }

        fn to_latex(expr: &Self::Expr) -> Option<String> {
            Some(expr.to_latex()) // if available; adjust to actual API[web:3][web:33]
        }
    }
}
