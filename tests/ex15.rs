#[cfg(test)]
mod ex12 {
    use matrix::{complex::Complex, vector::Vector, traits::{Lerp, Dot, Norm, Transpose}, matrix::Matrix};

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

    #[test]
    fn complex_ex06() {
        let c1 = Complex::new(1., 0.);
        let c2 = Complex::new(2., 0.);
        let c3 = Complex::new(3., 0.);
        let c4 = Complex::new(4., 0.);
        let c5 = Complex::new(5., 0.);
        let c6 = Complex::new(6., 0.);

        let u = Vector::from([c1, c2, c3]);
        let v = Vector::from([c4, c5, c6]);

        assert_eq!(u.cross(&v).to_string(), "[-3+0i, 6+0i, -3+0i]")
    }

    #[test]
    fn complex_ex07() {
        let c1 = Complex::new(1., 0.);
        let c2 = Complex::new(2., 0.);
        let c3 = Complex::new(3., 0.);
        let c4 = Complex::new(4., 0.);
        let c5 = Complex::new(5., 0.);
        let c6 = Complex::new(7., 0.);
        let c7 = Complex::new(0., 0.);
        let c8 = Complex::new(-2., 0.);
        let c9 = Complex::new(-3., 0.);
        let c10 = Complex::new(-5., 0.);
        let c11 = Complex::new(-9., 0.);

        let u = Matrix::from([[c1, c2], [c3, c4], [c8, c1]]);
        let v = Matrix::from([[c5, c9, c1, c4], [c11, c6, c7, c10]]);

        assert_eq!(
            u.dot(&v).to_string(),
            "[-13+0i, 11+0i, 1+0i, -6+0i]\n[-21+0i, 19+0i, 3+0i, -8+0i]\n[-19+0i, 13+0i, -2+0i, -13+0i]\n"
        );
    }

    #[test]
    fn complex_ex08() {
        let c1 = Complex::new(1., 6.);
        let c2 = Complex::new(2., 0.);
        let c3 = Complex::new(3., 0.);
        let c4 = Complex::new(4., 0.);
        let c5 = Complex::new(5., 9.);
        let c6 = Complex::new(7., 0.);
        let c7 = Complex::new(0., 0.);
        let c8 = Complex::new(-2., 0.);
        let c9 = Complex::new(-3., -4.);

        let u = Matrix::from([[c1, c2, c3], [c4, c5, c6], [c7, c8, c9]]);

        assert_eq!(u.trace().to_string(), "3+11i")
    }

    #[test]
    fn complex_ex09() {
        let c1 = Complex::new(9., 19.);
        let c2 = Complex::new(-21., 49.);
        let c3 = Complex::new(567., -43.);
        let c4 = Complex::new(48., 7.);
        let c5 = Complex::new(5., -9.);
        let c6 = Complex::new(-834., 433.);
        let c7 = Complex::new(4., 348.);
        let c8 = Complex::new(-2., -498.);
        let c9 = Complex::new(394., 934.);
        let c10 = Complex::new(7684., -10.);
        let c11 = Complex::new(94., 42.);
        let c12 = Complex::new(13., -748.);

        let u = Matrix::from([
            [c1, c2, c3],
            [c4, c5, c6],
            [c7, c8, c9],
            [c10, c11, c12]
        ]);
        let v = u.transpose();

        let result = Matrix::from([
            [c1, c4, c7, c10],
            [c2, c5, c8, c11],
            [c3, c6, c9, c12]
        ]);
        assert_eq!(v, result);
    }

    #[test]
    fn complex_ex10() {
        let c1 = Complex::new(2., 0.);
        let c2 = Complex::new(4., 0.);
        let c3 = Complex::new(6., 0.);
        let c4 = Complex::new(8., 0.);
        let c5 = Complex::new(0., 0.);

        let u = Matrix::from([
            [c1, c2, c3, c4],
            [c5, c5, c1, c2],
            [c1, c2, c3, c4],
        ]);
        assert_eq!(
            u.row_echelon().to_string(),
            "[1+0i, 2+0i, 0+0i, -2+0i]\n[0+0i, 0+0i, 1+0i, 2+0i]\n[0+0i, 0+0i, 0+0i, 0+0i]\n"
        )
    }

    #[test]
    fn complex_ex11() {
        let c1 = Complex::new(8., 0.);
        let c2 = Complex::new(5., 0.);
        let c3 = Complex::new(-2., 0.);
        let c4 = Complex::new(4., 0.);
        let c5 = Complex::new(7., 0.);
        let c6 = Complex::new(20., 0.);
        let c7 = Complex::new(7., 0.);
        let c8 = Complex::new(6., 0.);
        let c9 = Complex::new(1., 0.);

        let u = Matrix::from([[c1, c2, c3], [c4, c5, c6], [c7, c8, c9]]);
        assert_eq!(u.determinant().to_string(), "-174+0i");
    }

    #[test]
    fn complex_ex12() -> Result<(), String> {
        let c1 = Complex::new(8., 0.);
        let c2 = Complex::new(5., 0.);
        let c3 = Complex::new(-2., 0.);
        let c4 = Complex::new(4., 0.);
        let c5 = Complex::new(7., 0.);
        let c6 = Complex::new(20., 0.);
        let c7 = Complex::new(7., 0.);
        let c8 = Complex::new(6., 0.);
        let c9 = Complex::new(1., 0.);

        let u = Matrix::from([[c1, c2, c3], [c4, c5, c6], [c7, c8, c9]]);
        assert_eq!(
            u.inverse()?.to_string(),
            "[0.64942527+0i, 0.09770115+0i, -0.6551724+0i]\n[-0.7816092+0i, -0.12643678+0i, 0.9655172+0i]\n[0.14367816+0i, 0.07471265+0i, -0.20689656-0i]\n"
        );
        Ok(())
    }

    #[test]
    fn complex_ex13() {
        let c1 = Complex::new(8., 0.);
        let c2 = Complex::new(5., 0.);
        let c3 = Complex::new(-2., 0.);
        let c4 = Complex::new(4., 0.);
        let c5 = Complex::new(7., 0.);
        let c6 = Complex::new(20., 0.);
        let c7 = Complex::new(7., 0.);
        let c8 = Complex::new(6., 0.);
        let c9 = Complex::new(1., 0.);
        let c10 = Complex::new(21., 0.);
        let c11 = Complex::new(18., 0.);
        let c12 = Complex::new(7., 0.);

        let u = Matrix::from([
            [c1, c2, c3],
            [c4, c5, c6],
            [c7, c8, c9],
            [c10, c11, c12],
        ]);
        assert_eq!(
            u.rank(),
            3
        );
    }
}
