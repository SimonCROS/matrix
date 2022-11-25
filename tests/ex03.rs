#[cfg(test)]
mod ex03 {
    use matrix::traits::Dot;
    use matrix::vector::Vector;

    #[test]
    fn vector_dot() {
        let v1 = Vector::from([3., 5., 1.]);
        let v2 = Vector::from([8., 1., 2.]);

        assert_eq!(v1.dot(&v2), 31.)
    }
}
