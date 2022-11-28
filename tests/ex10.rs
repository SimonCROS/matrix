#[cfg(test)]
mod ex10 {
    use matrix::matrix::Matrix;

    #[test]
    fn row_echelon_identity() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        )
    }

    #[test]
    fn row_echelon_swap_1234() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 0.], [0., 1.]]))
    }

    #[test]
    fn row_echelon_swap_1224() {
        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 2.], [0., 0.]]))
    }

    #[test]
    fn row_echelon_example() {
        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1., 0.625, 0., 0., -12.1666667],
                [0., 0., 1., 0., -3.6666665],
                [0., 0., 0., 1., 29.499998],
            ])
        )
    }

    #[test]
    fn row_echelon_swap_1_mid() {
        let u = Matrix::from([
            [2., 4., 6., 8.,],
            [0., 0., 2., 4.],
            [2., 4., 6., 8.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1., 2., 0., -2.],
                [0., 0., 1., 2.],
                [0., 0., 0., 0.],
            ])
        )
    }

    #[test]
    fn row_echelon_swap_1() {
        let u = Matrix::from([
            [0., 0., 0., 1.],
            [1., 1., 1., 1.],
            [1., 1., 1., 1.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1., 1., 1., 0.],
                [0., 0., 0., 1.],
                [0., 0., 0., 0.],
            ])
        )
    }

    #[test]
    fn row_echelon_swap_2() {
        let u = Matrix::from([
            [0., 0., 2., 4.],
            [0., 2., 4., 6.],
            [2., 4., 6., 8.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1., 0., 0., 0.],
                [0., 1., 0., -1.],
                [0., 0., 1., 2.],
            ])
        )
    }

    #[test]
    fn row_echelon_empty_col() {
        let u = Matrix::from([
            [0., 0., 0., 2.],
            [0., 0., 2., 4.],
            [0., 2., 4., 6.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ])
        )
    }

    #[test]
    fn row_echelon_empty() {
        let u = Matrix::from([
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
            ])
        )
    }
}
