use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Operation {
    name: &'static str,
    path: &'static str,
    method: &'static str,
    description: &'static str,
}

#[derive(Serialize)]
struct Capabilities {
    version: &'static str,
    operations: Vec<Operation>,
}

async fn capabilities() -> Json<Capabilities> {
    Json(Capabilities {
        version: "v1",
        operations: vec![
            Operation {
                name: "simplify",
                path: "/v1/symbolic/simplify",
                method: "POST",
                description: "Simplify algebraic expression",
            },
            Operation {
                name: "differentiate",
                path: "/v1/symbolic/differentiate",
                method: "POST",
                description: "Differentiate expression with respect to a variable",
            },
            Operation {
                name: "evaluate",
                path: "/v1/symbolic/evaluate",
                method: "POST",
                description: "Evaluate expression given numeric variable assignments",
            },
            Operation {
                name: "matrix_solve",
                path: "/v1/numeric/matrix/solve",
                method: "POST",
                description: "Solve linear system A x = b",
            },
        ],
    })
}

pub fn router() -> Router {
    Router::new().route("/capabilities", get(capabilities))
}
