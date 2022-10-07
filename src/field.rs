use std::ops::{Add, Sub};

pub trait Field = Add<Output = Self> + Sub<Output = Self> + Clone + Sized;
