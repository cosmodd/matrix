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
{}

impl Field for f32 {}