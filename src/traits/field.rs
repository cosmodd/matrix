use std::{cmp, fmt};

pub trait Field:
    Sized
    + Copy
    + fmt::Display
    + fmt::Debug
    + cmp::PartialEq
{}

impl Field for f32 {}
impl Field for f64 {}