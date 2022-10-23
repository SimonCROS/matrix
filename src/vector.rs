use crate::field::{Dot, Field, SclAssign, Scl};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vector<const SIZE: usize, K: Field>(pub(super) [K; SIZE]);

impl<const SIZE: usize, K: Field> Vector<SIZE, K> {
    pub fn size(&self) -> usize {
        SIZE
    }
}

impl<const SIZE: usize, K: Field> From<[K; SIZE]> for Vector<SIZE, K> {
    fn from(content: [K; SIZE]) -> Self {
        Self(content)
    }
}

impl<const SIZE: usize, K: Field + Default> Default for Vector<SIZE, K> {
    fn default() -> Self {
        Self([(); SIZE].map(|_| K::default()))
    }
}

impl<const SIZE: usize, K: Field + Display + Debug> Display for Vector<SIZE, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)?;
        Ok(())
    }
}

impl<const SIZE: usize, K: Field> Add for Vector<SIZE, K> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl<const SIZE: usize, K: Field> Sub for Vector<SIZE, K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

impl<const SIZE: usize, K: Field> Scl<K> for Vector<SIZE, K> {
    type Output = Self;

    fn scl(self, other: K) -> Self::Output {
        let mut result = self;
        result.scl_assign(other);
        result
    }
}

impl<const SIZE: usize, K: Field + Default> Dot for Vector<SIZE, K> {
    type Output = K;

    fn dot(self, other: Self) -> Self::Output {
        self.0
            .into_iter()
            .zip(other.0)
            .fold(K::default(), |acc, (v1, v2)| acc + (v1 * v2))
    }
}

impl<const SIZE: usize, K: Field + Default> Dot for &Vector<SIZE, K> {
    type Output = K;

    fn dot(self, other: Self) -> Self::Output {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(K::default(), |acc, (v1, v2)| acc + (*v1 * *v2))
    }
}

impl<const SIZE: usize, K: Field> AddAssign for Vector<SIZE, K> {
    fn add_assign(&mut self, other: Self) {
        for cell in self.0.iter_mut().zip(other.0.into_iter()) {
            *cell.0 += cell.1;
        }
    }
}

impl<const SIZE: usize, K: Field> AddAssign<&Vector<SIZE, K>> for Vector<SIZE, K> {

    fn add_assign(&mut self, other: &Vector<SIZE, K>) {
        for cell in self.0.iter_mut().zip(other.0.into_iter()) {
            *cell.0 += cell.1;
        }
    }
}

impl<const SIZE: usize, K: Field> SubAssign for Vector<SIZE, K> {
    fn sub_assign(&mut self, other: Self) {
        for cell in self.0.iter_mut().zip(other.0.into_iter()) {
            *cell.0 -= cell.1;
        }
    }
}

impl<const SIZE: usize, K: Field> SubAssign<&Vector<SIZE, K>> for Vector<SIZE, K> {
    fn sub_assign(&mut self, other: &Vector<SIZE, K>) {
        for cell in self.0.iter_mut().zip(other.0.into_iter()) {
            *cell.0 -= cell.1;
        }
    }
}

impl<const SIZE: usize, K: Field> SclAssign<K> for Vector<SIZE, K> {
    fn scl_assign(&mut self, other: K) {
        for cell in &mut self.0 {
            *cell *= other;
        }
    }
}

impl<const SIZE: usize, K: Field> SclAssign<&K> for Vector<SIZE, K> {
    fn scl_assign(&mut self, other: &K) {
        for cell in &mut self.0 {
            *cell *= *other;
        }
    }
}
