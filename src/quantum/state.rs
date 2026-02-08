use nalgebra::{DVector, Complex};

pub type C64 = Complex<f64>;

#[derive(Debug, Clone)]
pub struct QState {
    pub amplitudes: DVector<C64>,
}

impl QState {
    pub fn from_real(data: &[f64]) -> Self {
        let norm: f64 = data.iter().map(|x| x * x).sum::<f64>().sqrt();
        let amps = DVector::from_iterator(
            data.len(),
            data.iter().map(|x| C64::new(*x / norm, 0.0)),
        );
        QState { amplitudes: amps }
    }
}
