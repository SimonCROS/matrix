pub mod matrix;

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

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
}
