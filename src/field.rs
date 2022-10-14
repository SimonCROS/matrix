use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

pub trait Field = Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + AddAssign + SubAssign + MulAssign + Clone + Sized;

pub trait Scl {
    type Rhs;
    type Output;

    fn scl(self, rhs: Self::Rhs) -> Self::Output;
}

pub trait SclAssign {
    type Rhs;

    fn scl_assign(&mut self, rhs: Self::Rhs);
}
