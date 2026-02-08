use crate::error::AppError;
use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

pub fn solve_linear_system(
    matrix: Vec<Vec<f64>>,
    rhs: Vec<f64>,
) -> Result<Vec<f64>, AppError> {
    let rows = matrix.len();
    if rows == 0 {
        return Err(AppError::BadRequest("Empty matrix".into()));
    }
    let cols = matrix[0].len();
    if rhs.len() != rows {
        return Err(AppError::BadRequest("RHS length mismatch".into()));
    }

    let mut a = Array2::zeros((rows, cols));
    for (i, row) in matrix.iter().enumerate() {
        if row.len() != cols {
            return Err(AppError::BadRequest("Non-rectangular matrix".into()));
        }
        for (j, val) in row.iter().enumerate() {
            a[(i, j)] = *val;
        }
    }

    let b = Array1::from(rhs);
    let x = a
        .solve_into(b)
        .map_err(|e| AppError::MathError(format!("Solve error: {}", e)))?;
    Ok(x.to_vec())
}
