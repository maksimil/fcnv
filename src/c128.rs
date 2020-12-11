use std::convert::From;
use std::ops::{Add, Div, Mul, Neg, Sub};
use svg2polylines::CoordinatePair;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

pub const I: Complex = Complex { x: 0.0, y: 1.0 };
pub const ZERO: Complex = Complex { x: 0.0, y: 0.0 };
pub const TPI: f64 = 6.28318530718;
pub const IN2P: Complex = Complex {
    x: 0.0,
    y: 0.15915494309,
};

impl Complex {
    pub fn ei(phi: f64) -> Complex {
        Complex {
            x: f64::cos(phi),
            y: f64::sin(phi),
        }
    }

    pub fn conj(&self) -> Complex {
        Complex {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn sabs(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn abs(&self) -> f64 {
        self.sabs().sqrt()
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

impl From<CoordinatePair> for Complex {
    fn from(cp: CoordinatePair) -> Self {
        Complex { x: cp.x, y: cp.y }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Complex {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            x: -self.x,
            y: -self.y,
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
