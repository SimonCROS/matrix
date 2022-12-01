#[cfg(test)]
mod ex11 {
    use matrix::matrix::Matrix;

    #[test]
    fn determinant_empty() {
        let u = Matrix::<0, 0, f32>::default();
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn determinant_oneone() {
        let u = Matrix::from([[5.]]);
        assert_eq!(u.determinant(), 5.0);
    }

    #[test]
    fn determinant_zero() {
        let u = Matrix::from([[1., -1.], [-1., 1.]]);
        assert_eq!(u.determinant(), 0.0);
    }

    #[test]
    fn determinant_diagonal_two() {
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(u.determinant(), 8.0);
    }

    #[test]
    fn determinant_example1() {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(u.determinant(), -174.0);
    }

    #[test]
    fn hard_3_3() {
        let u = Matrix::from([
            [2., 5., 2.],
            [4., 2.5, 20.],
            [3., 9., 4.],
        ]);
        assert_eq!(u.determinant(), -63.0);
    }

    #[test]
    fn hard_4_4() {
        let u = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_eq!(u.determinant(), 1032.0);
    }

    #[test]
    fn hard_5_5() {
        let u = Matrix::from([
            [8., 5., -2., 4., 6.],
            [4., 2.5, 20., 4., 5.],
            [8., 5., 1., 4., 7.],
            [28., -4., 17., 1., 6.],
            [57., -12., 7., 6., 4.],
        ]);
        assert_eq!(u.determinant(), 21523.0);
    }
}
