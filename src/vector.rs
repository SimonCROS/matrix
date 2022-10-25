use crate::field::{Dot, Field, Scl, SclAssign};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vector<const SIZE: usize, K>(pub(super) [K; SIZE]);

impl<const SIZE: usize, K> Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity `O(1)`
    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn linear_combination<const LEN: usize>(
        u: [Vector<SIZE, K>; LEN],
        coefs: [K; LEN],
    ) -> Vector<SIZE, K> {
        let mut iter = u.into_iter().zip(coefs);

        let mut acc = iter.next().map(|i| i.0.scl(i.1)).unwrap_or_default();
        for i in iter {
            acc.add_assign(i.0.scl(i.1))
        }
        acc
    }
}

impl<const SIZE: usize, K> From<[K; SIZE]> for Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity `O(1)`
    fn from(content: [K; SIZE]) -> Self {
        Self(content)
    }
}

impl<const SIZE: usize, K> Default for Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity `O(n)`
    fn default() -> Self {
        Self([(); SIZE].map(|_| K::default()))
    }
}

impl<const SIZE: usize, K> Display for Vector<SIZE, K>
where
    K: Field + Display + Debug,
{
    /// Complexity `O(n)`
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)?;
        Ok(())
    }
}

impl<const SIZE: usize, K> Add<Vector<SIZE, K>> for Vector<SIZE, K>
where
    K: Field,
{
    type Output = Vector<SIZE, K>;

    /// Complexity `O(n)`
    fn add(self, other: Vector<SIZE, K>) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl<const SIZE: usize, K> Sub<Vector<SIZE, K>> for Vector<SIZE, K>
where
    K: Field,
{
    type Output = Vector<SIZE, K>;

    /// Complexity `O(n)`
    fn sub(self, other: Vector<SIZE, K>) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

impl<const SIZE: usize, K> Scl<K> for Vector<SIZE, K>
where
    K: Field,
{
    type Output = Vector<SIZE, K>;

    /// Complexity `O(n)`
    fn scl(self, other: K) -> Self::Output {
        let mut result = self;
        result.scl_assign(other);
        result
    }
}

impl<const SIZE: usize, K> Dot<Vector<SIZE, K>> for Vector<SIZE, K>
where
    K: Field,
{
    type Output = K;

    /// Complexity: `O(n)`
    fn dot(self, other: Vector<SIZE, K>) -> Self::Output {
        self.0
            .into_iter()
            .zip(other.0)
            .fold(K::default(), |acc, (v1, v2)| acc + (v1 * v2))
    }
}

impl<const SIZE: usize, K> AddAssign<Vector<SIZE, K>> for Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity: `O(n)`
    fn add_assign(&mut self, other: Vector<SIZE, K>) {
        for cell in self.0.iter_mut().zip(other.0.into_iter()) {
            *cell.0 += cell.1;
        }
    }
}

impl<const SIZE: usize, K> SubAssign<Vector<SIZE, K>> for Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity: `O(n)`
    fn sub_assign(&mut self, other: Vector<SIZE, K>) {
        for cell in self.0.iter_mut().zip(other.0.into_iter()) {
            *cell.0 -= cell.1;
        }
    }
}

impl<const SIZE: usize, K> SclAssign<K> for Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity: `O(n)`
    fn scl_assign(&mut self, other: K) {
        for cell in &mut self.0 {
            *cell *= other;
        }
    }
}
