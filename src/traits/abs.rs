pub trait Abs {
    fn abs(self) -> Self;
}

macro_rules! impl_abs_float {
    ($($t:ty),*) => {
        $(
        impl Abs for $t {
            fn abs(self) -> Self {
                if self < 0.0 {
                    return -self
                }
                self
            }
        }
        )*
    };
}

impl_abs_float!(f32, f64);

macro_rules! impl_abs_int {
    ($($t:ty),*) => {
        $(
        impl Abs for $t {
            fn abs(self) -> Self {
                if self < 0 {
                    return -self
                }
                self
            }
        }
        )*
    };
}

impl_abs_int!(i8, i16, i32, i64, i128);

macro_rules! impl_abs_uint {
    ($($t:ty),*) => {
        $(
        impl Abs for $t {
            fn abs(self) -> Self {
                self
            }
        }
        )*
    };
}

impl_abs_uint!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_abs_uint {
        ($i:ident, $t:ty) => {
            #[test]
            fn $i() {
                assert_eq!(Abs::abs(0 as $t), 0 as $t);
                assert_eq!(Abs::abs(5 as $t), 5 as $t);
                assert_eq!(Abs::abs(42 as $t), 42 as $t);
                assert_eq!(Abs::abs(<$t>::MAX), <$t>::MAX);
            }
        };
    }

    test_abs_uint!(abs_u8, u8);
    test_abs_uint!(abs_u16, u16);
    test_abs_uint!(abs_u32, u32);
    test_abs_uint!(abs_u64, u64);
    test_abs_uint!(abs_u128, u128);

    macro_rules! test_abs_int {
        ($i:ident, $t:ty) => {
            #[test]
            fn $i() {
                assert_eq!(Abs::abs(0 as $t), 0 as $t);
                assert_eq!(Abs::abs(5 as $t), 5 as $t);
                assert_eq!(Abs::abs(-5 as $t), 5 as $t);
                assert_eq!(Abs::abs(42 as $t), 42 as $t);
                assert_eq!(Abs::abs(-42 as $t), 42 as $t);
                assert_eq!(Abs::abs(<$t>::MIN + 1), <$t>::MAX);
            }
        };
    }

    test_abs_int!(abs_i8, i8);
    test_abs_int!(abs_i16, i16);
    test_abs_int!(abs_i32, i32);
    test_abs_int!(abs_i64, i64);
    test_abs_int!(abs_i128, i128);

    macro_rules! test_abs_float {
        ($i:ident, $t:ty) => {
            #[test]
            fn $i() {
                assert_eq!(Abs::abs(0 as $t), 0 as $t);
                assert_eq!(Abs::abs(5 as $t), 5 as $t);
                assert_eq!(Abs::abs(-5 as $t), 5 as $t);
                assert_eq!(Abs::abs(42 as $t), 42 as $t);
                assert_eq!(Abs::abs(-42 as $t), 42 as $t);
                assert_eq!(Abs::abs(3.1415 as $t), 3.1415 as $t);
                assert_eq!(Abs::abs(-3.1415 as $t), 3.1415 as $t);
                assert_eq!(Abs::abs(<$t>::MIN), <$t>::MAX);
            }
        }
    }

    test_abs_float!(abs_f32, f32);
    test_abs_float!(abs_f64, f64);
}
