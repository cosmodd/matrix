use std::ops::{Add, Div, Mul, Neg, Sub};

trait Field:
    Copy +
    PartialEq +
    Add<Output = Self> +
    Sub<Output = Self> +
    Div<Output = Self> +
    Mul<Output = Self> +
    Neg<Output = Self>
{}

impl Field for f32 {}