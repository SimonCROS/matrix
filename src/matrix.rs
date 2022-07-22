use std::fmt::{self, Display, Formatter};

pub struct Matrix<const ROWS: usize, const COLS: usize, K>([[K; COLS]; ROWS]);

impl<const ROWS: usize, const COLS: usize, K> Matrix<ROWS, COLS, K> {
    pub fn new(content: [[K; COLS]; ROWS]) -> Self {
        Self(content)
    }

    /// Returns the size of the matrix in a tuple
    /// (rows: usize, cols: usize)
    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub fn is_square(&self) -> bool {
        ROWS == COLS
    }
}

impl<const ROWS: usize, const COLS: usize, K: Default> Default for Matrix<ROWS, COLS, K> {
    fn default() -> Self {
        Self([[(); COLS]; ROWS].map(|row| row.map(|_| K::default())))
    }
}

impl<const ROWS: usize, const COLS: usize, K: Display> Display for Matrix<ROWS, COLS, K> {
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
        assert_eq!(matrix.to_string(), "|    0        0        0    |\n|    0        0        0    |\n");
    }

    #[test]
    fn simple_values() {
        let matrix = Matrix::new([[1,2,3],[4,5,6]]);
        assert_eq!(matrix.to_string(), "|    1        2        3    |\n|    4        5        6    |\n");
    }

    #[test]
    fn square() {
        let matrix = Matrix::<4, 3, i32>::default();
        assert_eq!(matrix.is_square(), false);

        let matrix = Matrix::<4, 4, i32>::default();
        assert_eq!(matrix.is_square(), true);
    }
}
