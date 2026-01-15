use crate::traits::{Abs, MulAdd, One, Sqrt, Zero};
use std::{fmt, ops};

pub trait Field:
    Sized
    + Copy
    + One
    + Zero
    + Default
    + MulAdd
    + fmt::Display
    + fmt::Debug
    + PartialEq
    + PartialOrd
    + Abs
    + Sqrt
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
{}

macro_rules! impl_field_float {
    ($($t:ty),*) => {
        $(
        impl Field for $t {}
        )*
    };
}

impl_field_float!(f32, f64);

macro_rules! impl_field_int {
    ($($t:ty),*) => {
        $(
        impl Field for $t {}
        )*
    };
}

impl_field_int!(i8, i16, i32, i64, i128);
impl_field_int!(u8, u16, u32, u64, u128);
