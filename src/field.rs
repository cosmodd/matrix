use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Field:
    Copy +
    Clone +
    Debug +
    PartialEq +
    Add<Output = Self> +
    Sub<Output = Self> +
    Div<Output = Self> +
    Mul<Output = Self> +
    Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Field for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}