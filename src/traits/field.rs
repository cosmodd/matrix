use std::{fmt, ops};

pub trait Field:
    Copy +
    Clone +
    PartialEq +
    fmt::Debug +
    fmt::Display +
    ops::Add<Output = Self> +
    ops::Sub<Output = Self> +
    ops::Div<Output = Self> +
    ops::Mul<Output = Self> +
    ops::Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl Field for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }
}