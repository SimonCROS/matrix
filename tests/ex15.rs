#[cfg(test)]
mod ex12 {
    use matrix::{complex::Complex, vector::Vector, traits::{Lerp, Dot, Norm}};

    #[test]
    fn complex_mul_conj() {
        let c = Complex::new(3., 2.);
        assert_eq!(c * c.conj(), Complex::new(13., 0.));
    }

    #[test]
    fn complex_mul_conj_assign() {
        let mut c1 = Complex::new(3., 2.);
        let c2 = c1.conj();
        c1 *= c2;
        assert_eq!(c1, Complex::new(13., 0.));
    }

    #[test]
    fn complex_div() {
        let c1 = Complex::new(20., -4.);
        let c2 = Complex::new(3., 2.);
        assert_eq!(c1 / c2, Complex::new(4., -4.));
    }

    #[test]
    fn complex_div_assign() {
        let mut c1 = Complex::new(20., -4.);
        let c2 = Complex::new(3., 2.);
        c1 /= c2;
        assert_eq!(c1, Complex::new(4., -4.));
    }

    #[test]
    fn float_complex_div() {
        let f1 = -3.;
        let c1 = Complex::new(5., 1.);
        assert_eq!(f1 / c1, Complex::new(-15. / 26., 3. / 26.));
    }

    #[test]
    fn complex_ex00() {
        let c1 = Complex::new(3., 2.);
        let c2 = Complex::new(7., -4.);
        let c3 = Complex::new(-2., 1.);
        let c4 = Complex::new(-8., 2.);

        let mut u = Vector::from([c1, c2]);
        let v = Vector::from([c3, c4]);
        u += v;
        assert_eq!(u.to_string(), "[1+3i, -1-2i]");
    }

    #[test]
    fn complex_ex01() {
        let c1 = Complex::new(3., 2.);
        let c2 = Complex::new(7., -4.);
        let c3 = Complex::new(-2., 1.);
        let c4 = Complex::new(-8., 2.);

        let v1 = Vector::from([c1, c2]);
        let v2 = Vector::from([c3, c4]);

        assert_eq!(Vector::linear_combination([v1, v2], [c3, c2]).to_string(), "[-18+14i, -58+61i]");
    }

    #[test]
    fn complex_ex02() {
        let c1 = Complex::new(0., -20.);
        let c2 = Complex::new(10., 20.);

        let v1 = Vector::from([c1]);
        let v2 = Vector::from([c2]);

        assert_eq!(v1.lerp(v2, 0.5).to_string(), "[5+0i]");
    }

    #[test]
    fn complex_ex03() {
        let c1 = Complex::new(3., 2.);
        let c2 = Complex::new(7., -4.);
        let c3 = Complex::new(-2., 1.);
        let c4 = Complex::new(-8., 2.);

        let v1 = Vector::from([c1, c2]);
        let v2 = Vector::from([c3, c4]);

        assert_eq!(v1.dot(&v2).to_string(), "-56+45i")
    }

    #[test]
    fn complex_ex04() {
        let c1 = Complex::new(1., 0.);
        let c2 = Complex::new(2., 0.);
        let c3 = Complex::new(3., 0.);

        let u = Vector::from([c1, c2, c3]);

        assert_eq!([u.norm_1(), u.norm(), u.norm_inf()], [6.0, 3.74165738, 3.0]);
    }

    #[test]
    fn complex_ex05() {
        let c1 = Complex::new(1., 0.);
        let c2 = Complex::new(2., 0.);
        let c3 = Complex::new(4., 0.);

        let u = Vector::from([c2, c1]);
        let v = Vector::from([c3, c2]);

        assert!(u.angle_cos(&v).norm() - 1.0 <= f32::EPSILON);
    }
}
