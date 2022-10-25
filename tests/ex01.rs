#[cfg(test)]
mod ex01 {
    use matrix::vector::Vector;

    #[test]
    fn identity_linear_combinaison() {
        let v1 = Vector::from([1., 0., 0.]);
        let v2 = Vector::from([0., 1., 0.]);
        let v3 = Vector::from([0., 0., 1.]);
        let result = Vector::from([10., -2., 0.5]);

        assert_eq!(Vector::linear_combination([v1, v2, v3], [10., -2., 0.5]), result);
    }

    #[test]
    fn basic_linear_combinaison() {
        let v1 = Vector::from([1, 2, 3]);
        let v2 = Vector::from([0, 10, -100]);
        let result = Vector::from([10, 0, 230]);

        assert_eq!(Vector::linear_combination([v1, v2], [10, -2]), result);
    }
}
