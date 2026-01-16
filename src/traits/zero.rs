#[allow(dead_code)]

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! impl_zero_float {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self { 0.0 }
            fn is_zero(&self) -> bool { *self == 0.0 }
        }
    };
}

impl_zero_float!(f32);
impl_zero_float!(f64);

macro_rules! impl_zero_int {
    ($($t:ty),*) => {
        $(
        impl Zero for $t {
            fn zero() -> Self { 0 }
            fn is_zero(&self) -> bool { *self == 0 }
        }
        )*
    };
}

impl_zero_int!(u8, u16, u32, u64, u128);
impl_zero_int!(i8, i16, i32, i64, i128);