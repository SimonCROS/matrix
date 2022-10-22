use super::vector::Vector;
use crate::field::{Field, Transpose, Dot};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<const ROWS: usize, const COLS: usize, K: Field>([Vector<COLS, K>; ROWS]);

impl<const ROWS: usize, const COLS: usize, K: Field> Matrix<ROWS, COLS, K> {
    /// Returns the size of the matrix in a tuple
    /// (rows: usize, cols: usize)
    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub const fn is_square(&self) -> bool {
        ROWS == COLS
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> From<[[K; COLS]; ROWS]>
    for Matrix<ROWS, COLS, K>
{
    fn from(content: [[K; COLS]; ROWS]) -> Self {
        Self(content.map(|row| Vector::from(row)))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field + Default> Default for Matrix<ROWS, COLS, K> {
    fn default() -> Self {
        Self([(); ROWS].map(|_| Vector::default()))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field + Display + Debug> Display
    for Matrix<ROWS, COLS, K>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Transpose for Matrix<ROWS, COLS, K> {
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

impl<const ROWS: usize, const COLS: usize, K: Field> Transpose for &Matrix<ROWS, COLS, K> {
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

impl<const ROWS: usize, const COLS: usize, K: Field> Add for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Sub for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, const OCOLS: usize, K: Field + Default>
    Mul<Matrix<COLS, OCOLS, K>> for Matrix<ROWS, COLS, K>
{
    type Output = Matrix<ROWS, OCOLS, K>;

    fn mul(self, other: Matrix<COLS, OCOLS, K>) -> Self::Output {
        let other = other.transpose();
        let mut i = 0;

        Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); OCOLS].map(|_| {
                j += 1;
                (&self.0[i - 1]).dot(&other.0[j - 1])
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, const OCOLS: usize, K: Field + Default>
    Mul<Matrix<COLS, OCOLS, K>> for &Matrix<ROWS, COLS, K>
{
    type Output = Matrix<ROWS, OCOLS, K>;

    fn mul(self, other: Matrix<COLS, OCOLS, K>) -> Self::Output {
        let other = other.transpose();
        let mut i = 0;

        Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); OCOLS].map(|_| {
                j += 1;
                (&self.0[i - 1]).dot(&other.0[j - 1])
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, const OCOLS: usize, K: Field + Default>
    Mul<&Matrix<COLS, OCOLS, K>> for Matrix<ROWS, COLS, K>
{
    type Output = Matrix<ROWS, OCOLS, K>;

    fn mul(self, other: &Matrix<COLS, OCOLS, K>) -> Self::Output {
        let other = other.transpose();
        let mut i = 0;

        Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); OCOLS].map(|_| {
                j += 1;
                (&self.0[i - 1]).dot(&other.0[j - 1])
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, const OCOLS: usize, K: Field + Default>
    Mul<&Matrix<COLS, OCOLS, K>> for &Matrix<ROWS, COLS, K>
{
    type Output = Matrix<ROWS, OCOLS, K>;

    fn mul(self, other: &Matrix<COLS, OCOLS, K>) -> Self::Output {
        let other = other.transpose();
        let mut i = 0;

        Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); OCOLS].map(|_| {
                j += 1;
                (&self.0[i - 1]).dot(&other.0[j - 1])
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> AddAssign for Matrix<ROWS, COLS, K> {
    fn add_assign(&mut self, other: Self) {
        self.0
            .iter_mut()
            .zip(other.0.into_iter())
            .for_each(|(v1, v2)| *v1 += v2)
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> SubAssign for Matrix<ROWS, COLS, K> {
    fn sub_assign(&mut self, other: Self) {
        self.0
            .iter_mut()
            .zip(other.0.into_iter())
            .for_each(|(v1, v2)| *v1 -= v2)
    }
}

impl<const SIZE: usize, K: Field> From<Vector<SIZE, K>> for Matrix<1, SIZE, K> {
    fn from(v: Vector<SIZE, K>) -> Self {
        Self([v])
    }
}
