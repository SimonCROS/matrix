use crate::complex::Complex;
use crate::traits::{Dot, Field};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Index, IndexMut};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vector<const SIZE: usize, K>(pub(super) [K; SIZE]);

impl<const SIZE: usize, K> Vector<SIZE, K>
where
    K: Field,
{
    /// Complexity `O(1)`
    pub fn size(&self) -> usize {
        SIZE
    }

    /// Complexity `O(n)`
    pub fn linear_combination<const LEN: usize>(
        u: [Vector<SIZE, K>; LEN],
        coefs: [K; LEN],
    ) -> Vector<SIZE, K> {
        let iter = u.into_iter().zip(coefs);
        iter.fold(Self::default(), |acc, x| acc + x.0.mul(x.1))
    }

    pub fn iter(&self) -> core::slice::Iter<K> {
        self.0.iter()
    }

    pub fn norm_1(&self) -> f32 {
        self.0.iter().fold(0.0, |acc, f| acc + (*f).norm())
    }

    pub fn norm(&self) -> f32 {
        self.0
            .iter()
            .fold(0.0, |acc, f| acc + (*f * *f).norm())
            .sqrt()
    }

    pub fn norm_inf(&self) -> f32 {
        self.0
            .iter()
            .map(|f| (*f).norm())
            .reduce(f32::max)
            .unwrap_or_default()
    }
}

impl<K> Vector<3, K>
where
    K: Field,
{
    pub fn cross(&self, v: &Self) -> Self {
        Self([
            (self.0[1] * v.0[2]) - (self.0[2] * v.0[1]),
            (self.0[2] * v.0[0]) - (self.0[0] * v.0[2]),
            (self.0[0] * v.0[1]) - (self.0[1] * v.0[0]),
        ])
    }
}

impl<const SIZE: usize, K> Vector<SIZE, K>
where
    K: Field + Div<f32, Output = f32>,
{
    pub fn angle_cos(&self, v: &Self) -> f32 {
        self.dot(v) / (self.norm() * v.norm())
    }
}

impl<const SIZE: usize> Vector<SIZE, Complex>
{
    pub fn angle_cos(&self, v: &Self) -> Complex {
        self.dot(v) / (self.norm() * v.norm())
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
        Self([(); SIZE].map(|_| K::zero()))
    }
}

impl<const SIZE: usize, K> Display for Vector<SIZE, K>
where
    K: Field + Display,
{
    /// Complexity `O(n)`
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.map(|f| f.to_string()).join(", "))?;
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

impl<const SIZE: usize, K, S> Mul<S> for Vector<SIZE, K>
where
    K: Field + MulAssign<S>,
    S: Field,
{
    type Output = Vector<SIZE, K>;

    fn mul(self, other: S) -> Self::Output {
        let mut result = self;
        result *= other;
        result
    }
}

impl<const SIZE: usize, K, S> Div<S> for Vector<SIZE, K>
where
    K: Field + DivAssign<S>,
    S: Field,
{
    type Output = Vector<SIZE, K>;

    fn div(self, other: S) -> Self::Output {
        let mut result = self;
        result /= other;
        result
    }
}

impl<const SIZE: usize, K> Dot<&Vector<SIZE, K>> for Vector<SIZE, K>
where
    K: Field,
{
    type Output = K;

    /// Complexity: `O(n)`
    fn dot(&self, other: &Vector<SIZE, K>) -> Self::Output {
        self.0
            .into_iter()
            .zip(other.0)
            .fold(K::zero(), |acc, (v1, v2)| acc + (v1 * v2))
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

impl<const SIZE: usize, K, S> MulAssign<S> for Vector<SIZE, K>
where
    K: Field + MulAssign<S>,
    S: Field,
{
    /// Complexity: `O(n)`
    fn mul_assign(&mut self, other: S) {
        for cell in &mut self.0 {
            *cell *= other;
        }
    }
}

impl<const SIZE: usize, K, S> DivAssign<S> for Vector<SIZE, K>
where
    K: Field + DivAssign<S>,
    S: Field,
{
    /// Complexity: `O(n)`
    fn div_assign(&mut self, other: S) {
        for cell in &mut self.0 {
            *cell /= other;
        }
    }
}

impl<const SIZE: usize, K> Index<usize> for Vector<SIZE, K>
where
    K: Field {
    type Output = K;
    fn index<'a>(&'a self, i: usize) -> &'a K {
        &self.0[i]
    }
}

impl<const SIZE: usize, K> IndexMut<usize> for Vector<SIZE, K>
where
    K: Field {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut K {
        &mut self.0[i]
    }
}
