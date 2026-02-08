pub mod backend;
pub mod parse;
pub mod simplify;
pub mod differentiate;
pub mod integrate;
pub mod evaluate;
pub mod matrix;
pub mod stats;

pub use backend::Ast;
pub use parse::parse_expression;
pub use simplify::simplify_expression;
pub use differentiate::differentiate_expression;
pub use evaluate::evaluate_expression;
pub use matrix::solve_linear_system;
