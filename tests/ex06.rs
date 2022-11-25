#[cfg(test)]
mod ex06 {
    use matrix::{vector::Vector};

    #[test]
    fn flat() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        
        assert_eq!(u.cross(&v), Vector::from([0., 1., 0.]))
    }

    #[test]
    fn one_to_six() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);

        assert_eq!(u.cross(&v), Vector::from([-3., 6., -3.]))
    }

    #[test]
    fn random() {
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);

        assert_eq!(u.cross(&v), Vector::from([17., -58., -16.]))
    }
}
