#[cfg(test)]
mod ex08 {
    use matrix::{matrix::Matrix};

    #[test]
    fn matrix_trace_identity() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);

        assert_eq!(u.trace(), 2.)
    }

    #[test]
    fn matrix_trace_r1() {
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);

        assert_eq!(u.trace(), 9.)
    }

    #[test]
    fn matrix_trace_r2() {
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);

        assert_eq!(u.trace(), -21.)
    }
}
