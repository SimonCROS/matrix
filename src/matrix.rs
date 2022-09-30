use super::Field;
use super::Vector;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
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

impl<const N: usize, K: Field + Clone> From<Vector<N, K>> for Matrix<N, 1, K> {
    fn from(v: Vector<N, K>) -> Self {
        v.0
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field + Default> Default for Matrix<ROWS, COLS, K> {
    fn default() -> Self {
        Self([[(); COLS]; ROWS].map(|row| row.map(|_| K::default())))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field + Display> Display for Matrix<ROWS, COLS, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.0 {
            write!(f, "|")?;
            for x in line {
                write!(f, " {:^7.3} ", x)?
            }
            writeln!(f, "|")?
        }
        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Add for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(r1, r2)| r1.zip(r2).map(|(c1, c2)| c1 + c2)))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Sub for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(r1, r2)| r1.zip(r2).map(|(c1, c2)| c1 - c2)))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Mul for Matrix<ROWS, COLS, K> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(r1, r2)| r1.zip(r2).map(|(c1, c2)| c1 * c2)))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Field> Div for Matrix<ROWS, COLS, K> {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(r1, r2)| r1.zip(r2).map(|(c1, c2)| c1 / c2)))
    }
}

#[cfg(test)]
mod basic {
    use super::Matrix;

    #[test]
    fn rows() {
        let matrix = Matrix::<4, 2, i32>::default();
        assert_eq!(matrix.size().0, 4);
    }

    #[test]
    fn cols() {
        let matrix = Matrix::<2, 3, i32>::default();
        assert_eq!(matrix.size().1, 3);
    }

    #[test]
    fn print() {
        let matrix = Matrix::<2, 3, i32>::default();
        assert_eq!(
            matrix.to_string(),
            "|    0        0        0    |\n|    0        0        0    |\n"
        );
    }

    #[test]
    fn simple_values() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            matrix.to_string(),
            "|    1        2        3    |\n|    4        5        6    |\n"
        );
    }

    #[test]
    fn square() {
        let matrix = Matrix::<4, 3, i32>::default();
        assert_eq!(matrix.is_square(), false);

        let matrix = Matrix::<4, 4, i32>::default();
        assert_eq!(matrix.is_square(), true);
    }

    #[test]
    fn addition() {
        let m1 = Matrix::<2, 2, i32>::default();
        let m2 = Matrix::<2, 2, i32>::default();
        let m3 = Matrix::<2, 2, i32>::default();

        assert_eq!(m1 + m2, m3);
    }
}
