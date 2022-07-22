use super::Matrix;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct Vector<const SIZE: usize, K>(Matrix<1, SIZE, K>);

impl<const SIZE: usize, K> Vector<SIZE, K> {
    pub fn new(content: [K; SIZE]) -> Self {
        Self(Matrix::new([content]))
    }

    pub fn size(&self) -> usize {
        SIZE
    }
}

impl<const SIZE: usize, K> Deref for Vector<SIZE, K> {
    type Target = Matrix<1, SIZE, K>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const SIZE: usize, K> DerefMut for Vector<SIZE, K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    // #[test]
    // fn simple_values() {
    //     let vector = Vector::new([[1,2,3]]);
    //     assert_eq!(vector.to_string(), "|    1        2        3    |\n");
    // }
}
