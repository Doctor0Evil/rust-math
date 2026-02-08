use crate::error::AppError;
use rusted_sci_the::ode::{solve_ivp, OdeSystem}; // according to crate’s API[web:36]

pub struct SimpleOdeSystem<F>
where
    F: Fn(f64, &[f64], &mut [f64]) + Send + Sync,
{
    pub f: F,
}

impl<F> OdeSystem for SimpleOdeSystem<F>
where
    F: Fn(f64, &[f64], &mut [f64]) + Send + Sync,
{
    fn eval(&self, t: f64, y: &[f64], dy: &mut [f64]) {
        (self.f)(t, y, dy);
    }
}

pub fn solve_simple_ode(
    y0: Vec<f64>,
    t0: f64,
    t1: f64,
    dt: f64,
) -> Result<Vec<(f64, Vec<f64>)>, AppError> {
    // Example: dy/dt = y (simple exponential growth)
    let sys = SimpleOdeSystem { f: |_, y, dy| dy[0] = y[0] };

    solve_ivp(&sys, t0, t1, dt, &y0)
        .map_err(|e| AppError::MathError(format!("ODE solve error: {}", e)))
}
