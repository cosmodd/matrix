#[allow(dead_code)]

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_one_float {
    ($t:ty) => {
        impl One for $t {
            fn one() -> Self { 1.0 }
            fn is_one(&self) -> bool { *self == 1.0 }
        }
    };
}

impl_one_float!(f32);
impl_one_float!(f64);

macro_rules! impl_one_int {
    ($($t:ty),*) => {
        $(
        impl One for $t {
            fn one() -> Self { 1 }
            fn is_one(&self) -> bool { *self == 1 }
        }
        )*
    };
}

impl_one_int!(u8, u16, u32, u64, u128);
impl_one_int!(i8, i16, i32, i64, i128);
