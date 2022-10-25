use crate::traits::{Dot, Field, Lerp, Scl, SclAssign};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

impl<T> Lerp<T> for T
where
    T: Field + MulAssign<f32>,
{
    type Output = Self;

    /// Complexity: `O(n)`
    fn lerp(self, other: Self, t: f32) -> Self::Output {
        self.add(other.sub(self).scl(t))
    }
}
