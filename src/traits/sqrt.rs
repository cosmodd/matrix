pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt_float {
    ($($t:ty),*) => {
        $(
        impl Sqrt for $t {
            fn sqrt(self) -> Self {
                self.powf(0.5)
            }
        }
        )*
    };
}

impl_sqrt_float!(f32, f64);

macro_rules! impl_sqrt_int {
    ($($t:ty),*) => {
        $(
        impl Sqrt for $t {
            fn sqrt(self) -> Self {
                (self as f64).powf(0.5) as Self
            }
        }
        )*
    }
}

impl_sqrt_int!(i8, i16, i32, i64, i128);
impl_sqrt_int!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::min;

    macro_rules! test_sqrt_int {
        ($i:ident, $t:ty, $max:expr) => {
            #[test]
            fn $i() {
                let limit: usize = min(100, ($max as f64).sqrt().floor() as usize);
                for i in 0..limit {
                    assert_eq!(Sqrt::sqrt((i * i) as $t), i as $t);
                }
            }
        }
    }

    test_sqrt_int!(sqrt_i8, i8, i8::MAX);
    test_sqrt_int!(sqrt_i16, i16, i16::MAX);
    test_sqrt_int!(sqrt_i32, i32, i32::MAX);
    test_sqrt_int!(sqrt_i64, i64, i64::MAX);
    test_sqrt_int!(sqrt_i128, i128, i128::MAX);
    test_sqrt_int!(sqrt_u8, u8, u8::MAX);
    test_sqrt_int!(sqrt_u16, u16, u16::MAX);
    test_sqrt_int!(sqrt_u32, u32, u32::MAX);
    test_sqrt_int!(sqrt_u64, u64, u64::MAX);
    test_sqrt_int!(sqrt_u128, u128, u128::MAX);

    macro_rules! test_sqrt_float {
        ($i:ident, $t:ty) => {
            #[test]
            fn $i() {
                for i in 0..1024 {
                    let value = (i as $t) / 16.0;
                    let squared = (value * value);
                    assert_eq!(Sqrt::sqrt(squared), <$t>::sqrt(squared));
                }
            }
        }
    }

    test_sqrt_float!(sqrt_f32, f32);
    test_sqrt_float!(sqrt_f64, f64);
}