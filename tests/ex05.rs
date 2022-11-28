#[cfg(test)]
mod ex05 {
    use matrix::{vector::Vector};

    #[test]
    fn one() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);

        assert!(u.angle_cos(&v) - 1.0 <= f32::EPSILON);
    }

    #[test]
    fn zero() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);

        assert!(u.angle_cos(&v) - 0.0 <= f32::EPSILON);
    }

    #[test]
    fn negative_one() {
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([ 1., -1.]);

        assert!(u.angle_cos(&v) - -1.0 <= f32::EPSILON);
    }

    #[test]
    fn second_one() {
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);

        assert!(u.angle_cos(&v) - 1.0 <= f32::EPSILON);
    }

    #[test]
    fn random() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);

        assert_eq!(u.angle_cos(&v), 0.9746318);
    }
}
