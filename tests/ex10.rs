#[cfg(test)]
mod ex10 {
    use matrix::matrix::Matrix;

    #[test]
    fn matrix_2_3_transpose() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        )
    }

    #[test]
    fn matrix_2_8_transpose() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1.0, 0.0], [0.0, 1.0]]))
    }

    #[test]
    fn matrix_4_3_transpose() {
        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1.0, 2.0], [0.0, 0.0]]))
    }

    #[test]
    fn matrix_4_4_transpose() {
        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1.0, 0.625, 0.0, 0.0, -12.1666667],
                [0.0, 0.0, 1.0, 0.0, -3.6666667],
                [0.0, 0.0, 0.0, 1.0, 29.5],
            ])
        )
    }
}
