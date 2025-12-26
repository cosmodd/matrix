use std::{fmt, ops};

pub trait Field:
    Sized
    + Copy
    + fmt::Display
    + fmt::Debug
    + PartialEq
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
{}

impl Field for f32 {}
impl Field for f64 {}