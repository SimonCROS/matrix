#[cfg(test)]
mod ex00 {
    use matrix::traits::SclAssign;
    use matrix::matrix::Matrix;
    use matrix::vector::Vector;

    #[test]
    fn vector_add_assign() {
        let mut u = Vector::from([2, 3]);
        let v = Vector::from([5, 7]);
        u += v;
        assert_eq!(u.to_string(), "[7, 10]");
    }

    #[test]
    fn vector_sub_assign() {
        let mut u = Vector::from([2, 3]);
        let v = Vector::from([5, 7]);
        u -= v;
        assert_eq!(u.to_string(), "[-3, -4]");
    }

    #[test]
    fn vector_scl_assign() {
        let mut u = Vector::from([2, 3]);
        u.scl_assign(2);
        assert_eq!(u.to_string(), "[4, 6]");
    }

    #[test]
    fn matrix_add_assign() {
        let mut u = Matrix::from([[1, 2], [3, 4]]);
        let v = Matrix::from([[7, 4], [-2, 2]]);
        u += v;
        assert_eq!(u.to_string(), "[8, 6]\n[1, 6]\n");
    }

    #[test]
    fn matrix_sub_assign() {
        let mut u = Matrix::from([[1, 2], [3, 4]]);
        let v = Matrix::from([[7, 4], [-2, 2]]);
        u -= v;
        assert_eq!(u.to_string(), "[-6, -2]\n[5, 2]\n");
    }

    #[test]
    fn matrix_scl_assign() {
        let mut u = Matrix::from([[1, 2], [3, 4]]);
        u.scl_assign(2);
        assert_eq!(u.to_string(), "[2, 4]\n[6, 8]\n");
    }
}
