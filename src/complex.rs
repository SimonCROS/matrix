use crate::traits::{Zero, One, Norm};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, DivAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    pub fn conj(&self) -> Self {
        Self::new(self.real, -self.imag)
    }
}

impl Zero for Complex { fn zero() -> Self { Self { real: 0., imag: 0. } } }

impl One for Complex { fn one() -> Self { Self { real: 1., imag: 0. } } }

impl Norm for Complex { fn norm(&self) -> f32 { (self.real.powi(2) + self.imag.powi(2)).sqrt() } }

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.conj()
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real * rhs,
            imag: self.real * rhs
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real
        }
    }
}

impl Mul<Complex> for f32 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self * rhs.real, // - 0. * rhs.imag,
            imag: self * rhs.imag  // + 0. * rhs.real
        }
    }
}

impl Div<f32> for Complex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let conj = rhs.conj();
        (self * conj) / (rhs * conj).real
    }
}

impl Div<Complex> for f32 {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        let conj = rhs.conj();
        (self * conj) / (rhs * conj).real
    } 
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl MulAssign<f32> for Complex {
    fn mul_assign(&mut self, rhs: f32) {
        self.real *= rhs;
        self.imag *= rhs;
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let tmp = self.real * rhs.real - self.imag * rhs.imag;
        self.imag = self.real * rhs.imag + self.imag * rhs.real;
        self.real = tmp;
    }
}

impl DivAssign<f32> for Complex {
    fn div_assign(&mut self, rhs: f32) {
        self.real /= rhs;
        self.imag /= rhs;
    }
}

impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let conj = rhs.conj();
        *self *= conj;
        *self /= (rhs * conj).real;
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{:+}i", self.real, self.imag)?;
        Ok(())
    }
}
