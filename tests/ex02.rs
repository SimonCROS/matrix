#[cfg(test)]
mod ex02 {
    use matrix::{matrix::Matrix, traits::Lerp, vector::Vector};

    #[test]
    fn lerp_float_zero() {
        assert_eq!(0f32.lerp(1., 0.), 0.)
    }

    #[test]
    fn lerp_float_one() {
        assert_eq!(0f32.lerp(1., 1.), 1.)
    }

    #[test]
    fn lerp_float_middle() {
        assert_eq!(0f32.lerp(1., 0.5), 0.5)
    }

    #[test]
    fn lerp_float() {
        assert_eq!(21f32.lerp(42., 0.3), 27.3)
    }

    #[test]
    fn lerp_vec() {
        assert_eq!(
            Vector::from([2., 1.]).lerp(Vector::from([4., 2.]), 0.3),
            Vector::from([2.6, 1.3])
        )
    }

    #[test]
    fn lerp_matrix() {
        assert_eq!(
            Matrix::from([[2., 1.], [3., 4.]]).lerp(Matrix::from([[20., 10.], [30., 40.]]), 0.5),
            Matrix::from([[11., 5.5], [16.5, 22.]])
        );
    }
}
