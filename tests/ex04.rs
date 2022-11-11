#[cfg(test)]
mod ex04 {
    use matrix::vector::Vector;

    #[test]
    fn vector_zero_norms() {
        let u = Vector::from([0., 0., 0.]);
        
        assert_eq!([u.norm_1(), u.norm(), u.norm_inf()], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn vector_positive_norm() {
        let u = Vector::from([1., 2., 3.]);

        assert_eq!([u.norm_1(), u.norm(), u.norm_inf()], [6.0, 3.74165738, 3.0]);
    }

    #[test]
    fn vector_mixed_norm() {
        let u = Vector::from([-1., -2.]);

        assert_eq!([u.norm_1(), u.norm(), u.norm_inf()], [3.0, 2.236067977, 2.0]);
    }
}
