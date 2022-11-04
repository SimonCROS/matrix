use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Field = Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Copy
    + Sized
    + Default;

pub trait Transpose<Rhs = Self> {
    type Output;

    fn transpose(self) -> Self::Output;
}

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Lerp<Rhs = Self> {
    type Output;

    fn lerp(self, other: Self, t: f32) -> Self::Output;
}
