use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div};

pub trait Field = Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Copy
    + Sized
    + Default;

pub trait Transpose<Rhs = Self> {
    type Output;

    fn transpose(&self) -> Self::Output;
}

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(&self, rhs: Rhs) -> Self::Output;
}

pub trait Lerp<Rhs = Self> {
    type Output;

    fn lerp(&self, other: Self, t: f32) -> Self::Output;
}

pub trait Norm {
    fn norm(&self) -> f32;
}

impl Norm for f32 {
    fn norm(&self) -> f32 {
        self.abs() as f32
    }
}

impl Norm for f64 {
    fn norm(&self) -> f32 {
        self.abs() as f32
    }
}

impl Norm for i32 {
    fn norm(&self) -> f32 {
        self.abs() as f32
    }
}

impl Norm for i64 {
    fn norm(&self) -> f32 {
        self.abs() as f32
    }
}
