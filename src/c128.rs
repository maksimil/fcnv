use std::convert::From;
use std::ops::{Add, Mul};
use svg2polylines::CoordinatePair;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

impl Complex {
    pub fn conj(mut self) -> Complex {
        self.y = -self.y;
        self
    }
}

impl From<CoordinatePair> for Complex {
    fn from(cp: CoordinatePair) -> Self {
        Complex { x: cp.x, y: cp.y }
    }
}

pub const I: Complex = Complex { x: 0.0, y: 1.0 };

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
