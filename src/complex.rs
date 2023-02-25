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

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real
        }
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

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        self.real = self.real * rhs.real - self.imag * rhs.imag;
        self.imag = self.real * rhs.imag + self.imag * rhs.real;
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}+{}i", self.real, self.imag)?;
        Ok(())
    }
}
