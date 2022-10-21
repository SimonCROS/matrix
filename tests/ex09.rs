#[cfg(test)]
mod ex09 {
    use matrix::field::Transpose;
    use matrix::matrix::Matrix;

    #[test]
    fn matrix_2_3_transpose() {
        let u = Matrix::from([[1, 2, 4], [3, 4, 4]]);
        let v = u.transpose();

        let result = Matrix::from([[1, 3], [2, 4], [4, 4]]);
        assert_eq!(v, result);
    }

    #[test]
    fn matrix_4_3_transpose() {
        let u = Matrix::from([[8, 3, 1], [4, 2, 6], [8, 2, 5], [1, 9, 7]]);
        let v = u.transpose();

        let result = Matrix::from([[8, 4, 8, 1], [3, 2, 2, 9], [1, 6, 5, 7]]);
        assert_eq!(v, result);
    }

    #[test]
    fn matrix_4_4_transpose() {
        let u = Matrix::from([[1, 2, 3, 0], [4, 5, 6, 0], [7, 8, 9, 0], [0, 0, 0, 1]]);
        let v = u.transpose();

        let result = Matrix::from([[1, 4, 7, 0], [2, 5, 8, 0], [3, 6, 9, 0], [0, 0, 0, 1]]);
        assert_eq!(v, result);
    }
}
