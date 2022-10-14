#[cfg(test)]
mod ex00 {
    use matrix::Vector;
    use matrix::Matrix;
    use std::ops::{AddAssign, SubAssign};

    #[test]
    fn vector_add_assign() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add_assign(v);
        assert_eq!(u.to_string(), "[7.0]\n[10.0]\n");
    }

    #[test]
    fn vector_sub_assign() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.sub_assign(v);
        assert_eq!(u.to_string(), "[-3.0]\n[-4.0]\n");
    }

    #[test]
    fn vector_scl_assign() {
        let mut u = Vector::from([2., 3.]);
        u.scl_assign(2.);
        assert_eq!(u.to_string(), "[4.0]\n[6.0]\n");
    }

    #[test]
    fn matrix_add_assign() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.add_assign(v);
        assert_eq!(u.to_string(), "[8.0, 6.0]\n[1.0, 6.0]\n");
    }

    #[test]
    fn matrix_sub_assign() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.sub_assign(v);
        assert_eq!(u.to_string(), "[-6.0, -2.0]\n[5.0, 2.0]\n");
    }

    #[test]
    fn matrix_scl_assign() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        // u.scl_assign();
        assert_eq!(u.to_string(), "[2.0, 4.0]\n[6.0, 8.0]\n");
    }
}
