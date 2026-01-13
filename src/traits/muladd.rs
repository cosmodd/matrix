pub trait MulAdd: Copy {
    fn mul_add(self, b: Self, c: Self) -> Self;
}

impl MulAdd for f32 {
    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        f32::mul_add(self, b, c)
    }
}

impl MulAdd for f64 {
    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        f64::mul_add(self, b, c)
    }
}

macro_rules! impl_muladd_int {
    ($($t:ty),*) => {
        $(
            impl MulAdd for $t {
                #[inline]
                fn mul_add(self, b: Self, c: Self) -> Self {
                    self.wrapping_mul(b).wrapping_add(c)
                }
            }
        )*
    };
}

impl_muladd_int!(i8, i16, i32, i64, i128, isize);
impl_muladd_int!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_muladd_float {
        ($i:ident,$t:ty) => {
            #[test]
            fn $i() {
                let (a, b, c): ($t, $t, $t) = (10.0, 20.0, 30.);
                assert_eq!(MulAdd::mul_add(a, b, c), <$t>::mul_add(a, b, c));
            }
        };
    }

    test_muladd_float!(muladd_f32, f32);
    test_muladd_float!(muladd_f64, f64);

    macro_rules! test_muladd_int {
        ($i:ident,$t:ty) => {
            #[test]
            fn $i() {
                let (a, b, c): ($t, $t, $t) = (1, 2, 3);
                assert_eq!(MulAdd::mul_add(a, b, c), a * b + c);
            }
        };
    }

    test_muladd_int!(muladd_u8, u8);
    test_muladd_int!(muladd_u16, u16);
    test_muladd_int!(muladd_u32, u32);
    test_muladd_int!(muladd_u64, u64);
    test_muladd_int!(muladd_u128, u128);
    test_muladd_int!(muladd_usize, usize);

    test_muladd_int!(muladd_i8, i8);
    test_muladd_int!(muladd_i16, i16);
    test_muladd_int!(muladd_i32, i32);
    test_muladd_int!(muladd_i64, i64);
    test_muladd_int!(muladd_i128, i128);
    test_muladd_int!(muladd_isize, isize);
}