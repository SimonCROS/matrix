use std::ops::{Add, Div, Mul, Sub};

pub trait Field = Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Clone + Sized;
