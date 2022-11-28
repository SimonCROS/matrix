use super::vector::Vector;
use crate::traits::{Dot, Field, Transpose};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<const ROWS: usize, const COLS: usize, K>([Vector<COLS, K>; ROWS])
where
    K: Field;

impl<const ROWS: usize, const COLS: usize, K> Matrix<ROWS, COLS, K>
where
    K: Field,
{
    /// Returns the size of the matrix in a tuple
    /// (rows: usize, cols: usize)
    pub const fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub const fn is_square(&self) -> bool {
        ROWS == COLS
    }
}

impl<const ROWS: usize, const COLS: usize, K> Matrix<ROWS, COLS, K>
where
    K: Field + Neg<Output = K> + Display + std::fmt::Debug,
    f32: Div<K, Output = K>,
{
    fn row_echelon_step(mat: &mut Self, row: usize, col: usize) {
        println!("Normalize row {0}, based on column {1}.", row, col);
        mat.0[row] *= 1. / mat.0[row].0[col];

        for mul_row in 0..ROWS {
            if mul_row == row {
                continue;
            }
            let scl = -mat.0[mul_row].0[col];
            println!(
                "Multiply row {0}, by {1} and add it to row {2}.",
                row, scl, mul_row
            );
            mat.0[row]
                .0
                .clone()
                .iter()
                .zip(mat.0[mul_row].0.iter_mut())
                .for_each(|f| *f.1 += *f.0 * scl);
        }
    }

    pub fn row_echelon(&self) -> Self {
        println!("Row-echelon of \n{}", self);
        let mut res = self.clone();
        for row in 0..ROWS {
            for col in 0..COLS {
                if res.0[row].0[col] == K::default() {
                    println!(
                        "Row {0} at column {1} is null, we need to swap the rows.",
                        row, col
                    );

                    for other_row in row..ROWS {
                        if res.0[other_row].0[col] != K::default() {
                            println!("The first nonzero element is at row {}", other_row);
                            res.0.swap(row, other_row);
                            break;
                        }
                    }
                }
                if res.0[row].0[col] != K::default() {
                    Self::row_echelon_step(&mut res, row, col);
                    break;
                }
            }
            println!("Row-echelon of of \n{}OK for row \n{}", res, row);
        }
        println!("");
        println!("");
        res
    }
}

impl<const ROWS: usize, K> Matrix<ROWS, ROWS, K>
where
    K: Field,
{
    pub fn trace(&self) -> K {
        let mut val = K::default();

        for i in 0..ROWS {
            val += self.0[i].0[i];
        }
        val
    }
}

impl<const ROWS: usize, const COLS: usize, K> From<[[K; COLS]; ROWS]> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn from(content: [[K; COLS]; ROWS]) -> Self {
        Self(content.map(Vector::from))
    }
}

impl<const SIZE: usize, K> From<Vector<SIZE, K>> for Matrix<1, SIZE, K>
where
    K: Field,
{
    fn from(v: Vector<SIZE, K>) -> Self {
        Self([v])
    }
}

impl<const ROWS: usize, const COLS: usize, K> Default for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn default() -> Self {
        Self([(); ROWS].map(|_| Vector::default()))
    }
}

impl<const ROWS: usize, const COLS: usize, K> Display for Matrix<ROWS, COLS, K>
where
    K: Field + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize, K> Transpose for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<COLS, ROWS, K>;

    fn transpose(&self) -> Self::Output {
        let mut i = 0;

        Matrix::<COLS, ROWS, K>([(); COLS].map(|_| {
            let mut j = 0;

            i += 1;
            Vector([(); ROWS].map(|_| {
                j += 1;
                self.0[j - 1].0[i - 1]
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, K> Add<Matrix<ROWS, COLS, K>> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<ROWS, COLS, K>;

    fn add(self, other: Matrix<ROWS, COLS, K>) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K> Sub<Matrix<ROWS, COLS, K>> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<ROWS, COLS, K>;

    fn sub(self, other: Matrix<ROWS, COLS, K>) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K, S> Mul<S> for Matrix<ROWS, COLS, K>
where
    K: Field + MulAssign<S>,
    S: Field,
{
    type Output = Matrix<ROWS, COLS, K>;

    fn mul(self, other: S) -> Self::Output {
        let mut result = self;
        result *= other;
        result
    }
}

impl<const ROWS: usize, const COLS: usize, K> Dot<&Vector<COLS, K>> for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Vector<ROWS, K>;

    fn dot(&self, other: &Vector<COLS, K>) -> Self::Output {
        let mut i = 0;

        Vector::<ROWS, K>([(); ROWS].map(|_| {
            i += 1;
            self.0[i - 1].dot(other)
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, const OCOLS: usize, K> Dot<&Matrix<COLS, OCOLS, K>>
    for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    type Output = Matrix<ROWS, OCOLS, K>;

    fn dot(&self, other: &Matrix<COLS, OCOLS, K>) -> Self::Output {
        let other = other.transpose();
        let mut i = 0;

        Matrix::<ROWS, OCOLS, K>([(); ROWS].map(|_| {
            i += 1;

            let mut j = 0;
            Vector([(); OCOLS].map(|_| {
                j += 1;
                self.0[i - 1].dot(&other.0[j - 1])
            }))
        }))
    }
}

impl<const ROWS: usize, const COLS: usize, K> AddAssign<Matrix<ROWS, COLS, K>>
    for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn add_assign(&mut self, other: Matrix<ROWS, COLS, K>) {
        self.0
            .iter_mut()
            .zip(other.0.into_iter())
            .for_each(|(v1, v2)| *v1 += v2)
    }
}

impl<const ROWS: usize, const COLS: usize, K> SubAssign<Matrix<ROWS, COLS, K>>
    for Matrix<ROWS, COLS, K>
where
    K: Field,
{
    fn sub_assign(&mut self, other: Matrix<ROWS, COLS, K>) {
        self.0
            .iter_mut()
            .zip(other.0.into_iter())
            .for_each(|(v1, v2)| *v1 -= v2)
    }
}

impl<const ROWS: usize, const COLS: usize, K, S> MulAssign<S> for Matrix<ROWS, COLS, K>
where
    K: Field + MulAssign<S>,
    S: Field,
{
    fn mul_assign(&mut self, other: S) {
        for line in &mut self.0 {
            line.mul_assign(other);
        }
    }
}
