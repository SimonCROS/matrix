use super::vector::Vector;
use crate::traits::{Dot, Field, Transpose};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Matrix<const ROWS: usize, const COLS: usize, K>([Vector<COLS, K>; ROWS])
where
    K: Field;

impl<const ROWS: usize, const COLS: usize, K> Matrix<ROWS, COLS, K>
where
    K: Field,
{
    /// Returns the size of the matrix in a tuple
    /// (rows: usize, cols: usize)
    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub const fn is_square(&self) -> bool {
        ROWS == COLS
    }
}

impl<const ROWS: usize, const COLS: usize, K> From<[[K; COLS]; ROWS]> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn from(content: [[K; COLS]; ROWS]) -> Self {
        Self(content.map(Vector::from))
    }
}

impl<const SIZE: usize, K> From<Vector<SIZE, K>> for Matrix<1, SIZE, K>
where
    K: Field,
{
    fn from(v: Vector<SIZE, K>) -> Self {
        Self([v])
    }
}

impl<const ROWS: usize, const COLS: usize, K> Default for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn default() -> Self {
        Self([(); ROWS].map(|_| Vector::default()))
    }
}

impl<const ROWS: usize, const COLS: usize, K> Display for Matrix<ROWS, COLS, K>
where
    K: Field + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize, K> Transpose for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<COLS, ROWS, K>;

    fn transpose(self) -> Self::Output {
        let mut i = 0;

        Matrix::<COLS, ROWS, K>([(); COLS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); ROWS].map(|_| {
                j += 1;
                self.0[j - 1].0[i - 1]
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, K> Add<Matrix<ROWS, COLS, K>> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<ROWS, COLS, K>;

    fn add(self, other: Matrix<ROWS, COLS, K>) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K> Sub<Matrix<ROWS, COLS, K>> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<ROWS, COLS, K>;

    fn sub(self, other: Matrix<ROWS, COLS, K>) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K, S> Mul<S> for Matrix<ROWS, COLS, K>
where
    K: Field + MulAssign<S>,
    S: Field,
{
    type Output = Matrix<ROWS, COLS, K>;

    fn mul(self, other: S) -> Self::Output {
        let mut result = self;
        result *= other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, const OCOLS: usize, K> Dot<Matrix<COLS, OCOLS, K>>
    for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<ROWS, OCOLS, K>;

    fn dot(self, other: Matrix<COLS, OCOLS, K>) -> Self::Output {
        let other = other.transpose();
        let mut i = 0;

        Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); OCOLS].map(|_| {
                j += 1;
                self.0[i - 1].dot(other.0[j - 1])
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, K> AddAssign<Matrix<ROWS, COLS, K>>
    for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn add_assign(&mut self, other: Matrix<ROWS, COLS, K>) {
        self.0
            .iter_mut()
            .zip(other.0.into_iter())
            .for_each(|(v1, v2)| *v1 += v2)
    }
}

impl<const ROWS: usize, const COLS: usize, K> SubAssign<Matrix<ROWS, COLS, K>>
    for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn sub_assign(&mut self, other: Matrix<ROWS, COLS, K>) {
        self.0
            .iter_mut()
            .zip(other.0.into_iter())
            .for_each(|(v1, v2)| *v1 -= v2)
    }
}

impl<const ROWS: usize, const COLS: usize, K, S> MulAssign<S> for Matrix<ROWS, COLS, K>
where
    K: Field + MulAssign<S>,
    S: Field,
{
    fn mul_assign(&mut self, other: S) {
        for line in &mut self.0 {
            line.mul_assign(other);
        }
    }
}
