use std::ops::{Add, Sub, Mul, Div};

pub trait Field: Add + Sub + Mul + Div + Clone + Sized {}

impl<T> Field for T
    where T: Add + Sub + Mul + Div + Clone + Sized {}
