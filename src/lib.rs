pub mod matrix;
pub mod vector;

#[cfg(test)]
mod basic_tests {
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn matrix_rows() {
        let matrix: Matrix<i32> = Matrix::new(2, 3);
        assert_eq!(matrix.size().0, 2);
    }

    #[test]
    fn matrix_cols() {
        let matrix: Matrix<i32> = Matrix::new(2, 3);
        assert_eq!(matrix.size().1, 3);
    }

    #[test]
    fn vector_size() {
        let matrix: Vector<i32> = Vector::new(3);
        assert_eq!(matrix.size(), 3);
    }
}
