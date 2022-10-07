use super::Field;
use super::Matrix;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[derive(Clone, Default, Debug)]
pub struct Vector<const SIZE: usize, K: Field>(pub(super) Matrix<SIZE, 1, K>);

impl<const SIZE: usize, K: Field> Vector<SIZE, K> {
    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn scl_assign(&mut self, _other: f32) {
        unimplemented!()
    }
}

impl<const SIZE: usize, K: Field> From<[K; SIZE]> for Vector<SIZE, K> {
    fn from(content: [K; SIZE]) -> Self {
        Self(content.map(|x| [x]).into())
    }
}

impl<const SIZE: usize, K: Field> From<Matrix<SIZE, 1, K>> for Vector<SIZE, K> {
    fn from(content: Matrix<SIZE, 1, K>) -> Self {
        Self(content)
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

impl<const SIZE: usize, K: Field + Display + Debug> Display for Vector<SIZE, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const SIZE: usize, K: Field> Add for Vector<SIZE, K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector::from(self.0 + other.0)
    }
}

impl<const SIZE: usize, K: Field> Sub for Vector<SIZE, K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector::from(self.0 - other.0)
    }
}

impl<const SIZE: usize, K: Field> AddAssign for Vector<SIZE, K> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<const SIZE: usize, K: Field> SubAssign for Vector<SIZE, K> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}
