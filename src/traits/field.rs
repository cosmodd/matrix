use std::fmt;

pub trait Field:
    Sized
    + Copy
    + fmt::Display
    + fmt::Debug
    + PartialEq
{}

impl Field for f32 {}
impl Field for f64 {}