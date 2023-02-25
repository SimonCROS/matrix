#[cfg(test)]
mod ex12 {
    use matrix::matrix::Matrix;

    #[test]
    fn determinant_zero() {
        match Matrix::from([[1., -1.], [-1., 1.]]).inverse() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn inverse_identity() -> Result<(), String> {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.inverse()?,
            Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],])
        );
        Ok(())
    }

    #[test]
    fn inverse_identity_double() -> Result<(), String> {
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(
            u.inverse()?,
            Matrix::from([[0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 0.5],])
        );
        Ok(())
    }

    #[test]
    fn inverse_example() -> Result<(), String> {
        let u = Matrix::from([
            [8.0f32, 5., -2.],
            [4., 7., 20.],
            [7., 6., 1.]]);
        assert_eq!(
            u.inverse()?,
            Matrix::from([
                [0.649425287, 0.097701149, -0.655172414],
                [-0.781609195, -0.126436782, 0.965517241],
                [0.143678161, 0.074712650, -0.206896552],
            ])
        );
        Ok(())
    }

    #[test]
    fn inverse_reverse_identity() -> Result<(), String> {
        let u = Matrix::from([
            [0.0f32, 0., 1.],
            [0., 1., 0.],
            [1., 0., 0.]]);
        assert_eq!(
            u.inverse()?,
            Matrix::from([
                [0.0f32, 0., 1.],
                [0., 1., 0.],
                [1., 0., 0.]])
        );
        Ok(())
    }
}
