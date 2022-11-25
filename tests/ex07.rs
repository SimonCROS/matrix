#[cfg(test)]
mod ex07 {
    use matrix::{matrix::Matrix, traits::Dot, vector::Vector};

    #[test]
    fn matrix_vector_mul_1() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);

        assert_eq!(u.dot(&v), Vector::from([4., 2.]))
    }

    #[test]
    fn matrix_mul_2_2() {
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);

        assert_eq!(u.dot(&v), Vector::from([8., 4.]))
    }

    #[test]
    fn matrix_vector_mul_2() {
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);

        assert_eq!(u.dot(&v), Vector::from([4., -4.]))
    }

    #[test]
    fn matrix_vector_mul_3() {
        let u = Matrix::from([[1, 2], [3, 4]]);
        let v = Matrix::from([[7, 4], [-2, 2]]);

        assert_eq!(u.dot(&v), Matrix::from([[3, 8], [13, 20]]))
    }

    #[test]
    fn matrix_mul_4_2() {
        let u = Matrix::from([[1, 2], [3, 4], [-2, 1], [8, 6]]);
        let v = Matrix::from([[5, -3, 1, 4], [-9, 7, 0, -5]]);

        assert_eq!(
            u.dot(&v),
            Matrix::from([
                [-13, 11, 1, -6],
                [-21, 19, 3, -8],
                [-19, 13, -2, -13],
                [-14, 18, 8, 2]
            ])
        )
    }

    #[test]
    fn matrix_mul_3_2() {
        let u = Matrix::from([[1, 2], [3, 4], [-2, 1]]);
        let v = Matrix::from([[5, -3, 1, 4], [-9, 7, 0, -5]]);

        assert_eq!(
            u.dot(&v),
            Matrix::from([[-13, 11, 1, -6], [-21, 19, 3, -8], [-19, 13, -2, -13]])
        )
    }

    #[test]
    fn matrix_mul_identity() {
        let u = Matrix::from([[1, 2, 3, 0], [4, 5, 6, 0], [7, 8, 9, 0], [0, 0, 0, 1]]);
        let v = Matrix::from([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);

        assert_eq!(u.dot(&v), u)
    }
}
