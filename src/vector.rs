use super::Field;
use super::Matrix;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Default)]
pub struct Vector<const SIZE: usize, K: Field>(pub(super) Matrix<SIZE, 1, K>);

impl<const SIZE: usize, K: Field> Vector<SIZE, K> {
    pub fn size(&self) -> usize {
        SIZE
    }
}

impl<const SIZE: usize, K: Field> From<[K; SIZE]> for Vector<SIZE, K> {
    fn from(content: [K; SIZE]) -> Self {
        Self(content.map(|x| [x]).into())
    }
}

impl<const SIZE: usize, K: Field> Deref for Vector<SIZE, K> {
    type Target = Matrix<SIZE, 1, K>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const SIZE: usize, K: Field> DerefMut for Vector<SIZE, K> {
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

    #[test]
    fn simple_values() {
        let vector = Vector::from([1, 2, 3]);
        assert_eq!(vector.to_string(), "|    1    |\n|    2    |\n|    3    |\n");
    }
}
