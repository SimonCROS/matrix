#![feature(array_zip)]
#![feature(trait_alias)]

mod matrix;
mod vector;
mod field;
mod tests;

pub use self::matrix::Matrix;
pub use self::vector::Vector;
pub use self::field::Field;
