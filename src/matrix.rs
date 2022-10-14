use super::Vector;
use crate::field::{Field, Scl, SclAssign};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<const ROWS: usize, const COLS: usize, K: Field>([[K; COLS]; ROWS]);

impl<const ROWS: usize, const COLS: usize, K: Field> Matrix<ROWS, COLS, K> {
    /// Returns the size of the matrix in a tuple
    /// (rows: usize, cols: usize)
    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub const fn is_square(&self) -> bool {
        ROWS == COLS
    }

    pub fn iter(&self) -> Iter<'_, [K; COLS]> {
        self.0.iter()
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> From<[[K; COLS]; ROWS]> for Matrix<ROWS, COLS, K> {
    fn from(content: [[K; COLS]; ROWS]) -> Self {
        Self(content)
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field + Default> Default for Matrix<ROWS, COLS, K> {
    fn default() -> Self {
        Self([[(); COLS]; ROWS].map(|row| row.map(|_| K::default())))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field + Display + Debug> Display for Matrix<ROWS, COLS, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.0 {
            writeln!(f, "{:?}", line)?;
        }
        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Add for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Sub for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(
            self.0
                .zip(other.0)
                .map(|(r1, r2)| r1.zip(r2).map(|(c1, c2)| c1 - c2)),
        )
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Mul for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Scl for Matrix<ROWS, COLS, K> {
    type Rhs = K;
    type Output = Self;

    fn scl(self, other: K) -> Self {
        unimplemented!()
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> AddAssign for Matrix<ROWS, COLS, K> {
    fn add_assign(&mut self, rhs: Self) {
        for line in self.0.iter_mut().zip(rhs.0.into_iter()) {
            for cell in line.0.iter_mut().zip(line.1.into_iter()) {
                *cell.0 += cell.1;
            }
        }
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> SubAssign for Matrix<ROWS, COLS, K> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> MulAssign for Matrix<ROWS, COLS, K> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> SclAssign for Matrix<ROWS, COLS, K> {
    type Rhs = K;

    fn scl_assign(&mut self, rhs: K) {
        *self = self.clone().scl(rhs);
    }
}

impl<const N: usize, K: Field + Clone> From<Vector<N, K>> for Matrix<N, 1, K> {
    fn from(v: Vector<N, K>) -> Self {
        v.0
    }
}
