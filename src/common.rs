use crate::traits::{Dot, Field, Lerp, Scl, SclAssign};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

impl<T> Lerp<T> for T
where
    T: Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self>,
{
    type Output = Self;

    /// Complexity: `O(n)`
    fn lerp(self, other: Self, t: f32) -> Self::Output {
        self.add(other.sub(self).mul(t))
    }
}

// let other = other.transpose();
// let mut i = 0;

// Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
//     let mut j = 0;

//     i += 1;
//     Vector([(); OCOLS].map(|_| {
//         j += 1;
//         self.0[i - 1].dot(other.0[j - 1])
//     }))
// }))
