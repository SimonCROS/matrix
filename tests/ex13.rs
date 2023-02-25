#[cfg(test)]
mod ex12 {
    use matrix::matrix::Matrix;

    #[test]
    fn rank_identity() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.rank(),
            3
        );
    }

    #[test]
    fn rank_example1() {
        let u = Matrix::from([
            [ 1., 2., 0., 0.],
            [ 2., 4., 0., 0.],
            [-1., 2., 1., 1.],
        ]);
        assert_eq!(
            u.rank(),
            2
        );
    }

    #[test]
    fn rank_example2() {
        let u = Matrix::from([
            [ 8., 5., -2.],
            [ 4., 7., 20.],
            [ 7., 6., 1.],
            [21., 18., 7.],
        ]);
        assert_eq!(
            u.rank(),
            3
        );
    }

    #[test]
    fn rank_empty() {
        let u = Matrix::from([
            [ 0., 0. ],
            [ 0., 0. ],
        ]);
        assert_eq!(
            u.rank(),
            0
        );
    }
}
