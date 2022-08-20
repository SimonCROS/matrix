use super::Field;
use super::Matrix;
use std::fmt::{self, Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Default)]
pub struct Vector<const SIZE: usize, K: Field>(Matrix<1, SIZE, K>);

impl<const SIZE: usize, K: Field> Vector<SIZE, K> {
    pub fn new(content: [K; SIZE]) -> Self {
        Self(Matrix::new([content]))
    }

    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn to_matrix<const ROWS: usize, const COLS: usize>(&self) -> Matrix<ROWS, COLS, K> {
        assert_eq!(ROWS * COLS, SIZE, "ROWS * COLS != SIZE");
        Matrix::new([[(); COLS]; ROWS].map(|x| x.map(|_| self.0 .0[0][0].clone())))
    }
}

impl<const SIZE: usize, K: Field> Deref for Vector<SIZE, K> {
    type Target = Matrix<1, SIZE, K>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const SIZE: usize, K: Field> DerefMut for Vector<SIZE, K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const SIZE: usize, K: Field + Display> Display for Vector<SIZE, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in self.iter() {
            write!(f, "(")?;

            let mut iterator = line.iter().enumerate().peekable();
            while let Some((_, x)) = iterator.next() {
                write!(f, "{:^.3}", x)?;
                if !iterator.peek().is_none() {
                    write!(f, ", ")?;
                }
            }

            writeln!(f, ")")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod basic {
    use super::Vector;

    #[test]
    fn size() {
        let vector = Vector::<4, i32>::default();
        assert_eq!(vector.size(), 4);
    }

    #[test]
    fn simple_values() {
        let vector = Vector::new([1, 2, 3]);
        assert_eq!(vector.to_string(), "(1, 2, 3)\n");
    }
}
