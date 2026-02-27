#![allow(dead_code)]

use crate::traits::{Abs, MulAdd, One, Sqrt, Zero};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Complex<K> {
    pub real: K,
    pub imag: K,
}

impl<K> Complex<K> {
    pub fn new(real: K, imag: K) -> Self {
        Self { real, imag }
    }
}

impl<K> Complex<K>
where K:
    ops::Mul<Output = K>
    + ops::Add<Output = K>
    + Copy
{
    pub fn magnitude_squared(&self) -> K {
        self.real * self.real + self.imag * self.imag
    }
}

impl<K> Complex<K>
where K:
    ops::Mul<Output = K>
    + ops::Add<Output = K>
    + Sqrt<Output = K>
    + Copy
{
    pub fn magnitude(&self) -> K {
        Sqrt::sqrt(self.magnitude_squared())
    }
}

impl<K> Display for Complex<K>
where K: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl<K> PartialEq for Complex<K>
where K: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl<K> PartialOrd for Complex<K>
where K: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.real != other.real {
            return self.real.partial_cmp(&other.real);
        }
        self.imag.partial_cmp(&other.imag)
    }
}

impl<K> ops::Add for Complex<K>
where K: ops::Add<Output = K> {
    type Output = Complex<K>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<K> ops::AddAssign for Complex<K>
where K: ops::AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl<K> ops::Sub for Complex<K>
where K: ops::Sub<Output = K> {
    type Output = Complex<K>;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl<K> ops::SubAssign for Complex<K>
where K: ops::SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl<K> ops::Mul<K> for Complex<K>
where K:
    ops::Mul<Output = K>
    + Copy
{
    type Output = Complex<K>;

    fn mul(self, rhs: K) -> Self::Output {
        Complex {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}

impl<K> ops::MulAssign<K> for Complex<K>
where K:
    ops::Mul<Output = K>
    + Copy
{
    fn mul_assign(&mut self, rhs: K) {
        self.real = self.real * rhs;
        self.imag = self.imag * rhs;
    }
}

impl<K> ops::Mul for Complex<K>
where K:
    ops::Neg<Output = K>
    + MulAdd
    + Zero
    + Copy
{
    type Output = Complex<K>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Complex::new(K::zero(), K::zero());

        result.real = MulAdd::mul_add(self.real, rhs.real, -MulAdd::mul_add(self.imag, rhs.imag, result.real));
        result.imag = MulAdd::mul_add(self.imag, rhs.real, MulAdd::mul_add(self.real, rhs.imag, result.imag));

        result
    }
}

impl<K> ops::MulAssign for Complex<K>
where K:
    ops::Neg<Output = K>
    + MulAdd
    + Zero
    + Copy
{
    fn mul_assign(&mut self, rhs: Self) {
        let temp_real = self.real;

        self.real = MulAdd::mul_add(temp_real, rhs.real, -MulAdd::mul_add(self.imag, rhs.imag, K::zero()));
        self.imag = MulAdd::mul_add(self.imag, rhs.real, MulAdd::mul_add(temp_real, rhs.imag, K::zero()));
    }
}

impl<K> MulAdd for Complex<K>
where K:
    MulAdd
    + ops::Neg<Output = K>
    + Zero
    + Copy
{
    fn mul_add(self, multiplier: Self, mut addend: Self) -> Self {
        addend.real = MulAdd::mul_add(self.real, multiplier.real, -MulAdd::mul_add(self.imag, multiplier.imag, addend.real));
        addend.imag = MulAdd::mul_add(self.imag, multiplier.real, MulAdd::mul_add(self.real, multiplier.imag, addend.imag));
        addend
    }
}


impl<K> ops::Div<K> for Complex<K>
where K:
    ops::Div<Output = K>
    + Copy
{
    type Output = Complex<K>;

    fn div(self, rhs: K) -> Self::Output {
        Complex {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

impl<K> ops::DivAssign<K> for Complex<K>
where K:
    ops::Div<Output = K>
    + Copy
{
    fn div_assign(&mut self, rhs: K) {
        self.real = self.real / rhs;
        self.imag = self.imag / rhs;
    }
}

impl<K> ops::Div for Complex<K>
where K:
    ops::Add<Output = K>
    + ops::Sub<Output = K>
    + ops::Mul<Output = K>
    + ops::Div<Output = K>
    + Copy
{
    type Output = Complex<K>;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs_mag_squared = rhs.magnitude_squared();

        Complex {
            real: (self.real * rhs.real + self.imag * rhs.imag) / rhs_mag_squared,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / rhs_mag_squared,
        }
    }
}

impl<K> ops::DivAssign for Complex<K>
where K:
    ops::Add<Output = K>
    + ops::Sub<Output = K>
    + ops::Mul<Output = K>
    + ops::Div<Output = K>
    + Copy
{
    fn div_assign(&mut self, rhs: Self) {
        let right_mag_squared = rhs.magnitude_squared();
        let temp_real = self.real;

        self.real = (temp_real * rhs.real + self.imag * rhs.imag) / right_mag_squared;
        self.imag = (self.imag * rhs.real - temp_real * rhs.imag) / right_mag_squared;
    }
}

impl<K> ops::Neg for Complex<K>
where K:
    ops::Neg<Output = K>
{
    type Output = Complex<K>;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<K> Abs for Complex<K>
where K:
    ops::Mul<Output = K>
    + ops::Add<Output = K>
    + Sqrt<Output = K>
    + Copy
{
    type Output = K;

    fn abs(self) -> Self::Output {
        self.magnitude()
    }
}

impl<K> Sqrt for Complex<K>
where K:
    ops::Add<Output = K>
    + ops::Sub<Output = K>
    + ops::Mul<Output = K>
    + ops::Div<Output = K>
    + ops::Neg<Output = K>
    + Sqrt<Output = K>
    + PartialOrd
    + One
    + Zero
    + Copy
{
    type Output = Self;

    fn sqrt(self) -> Self {
        let two = K::one() + K::one();
        let mag = self.magnitude();
        let mut i_sign = K::one();
        if self.imag < K::zero() {
            i_sign = -K::one();
        }

        Complex {
            real: Sqrt::sqrt((mag + self.real) / two),
            imag: i_sign * Sqrt::sqrt((mag - self.real) / two),
        }
    }
}

impl<K> Zero for Complex<K>
where K:
    Zero + PartialEq
{
    fn zero() -> Self {
        Complex::new(K::zero(), K::zero())
    }

    fn is_zero(&self) -> bool {
        *self == Complex::zero()
    }
}

impl<K> One for Complex<K>
where K:
    One
    + Zero
    + PartialEq
{
    fn one() -> Self {
        Complex::new(K::one(), K::zero())
    }

    fn is_one(&self) -> bool {
        *self == Complex::one()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut a = Complex::new(1.0, 2.0) + Complex::new(3.0, 4.0);
        assert_eq!(a, Complex::new(4.0, 6.0));

        a += Complex::new(5.0, 6.0);
        assert_eq!(a, Complex::new(9.0, 12.0));
    }

    #[test]
    fn test_substraction() {
        let mut a = Complex::new(1.0, 2.0) - Complex::new(3.0, 4.0);
        assert_eq!(a, Complex::new(-2.0, -2.0));

        a -= Complex::new(-1.0, -2.0);
        assert_eq!(a, Complex::new(-1.0, 0.0));
    }

    #[test]
    fn test_multiplication_by_scalar() {
        let mut a = Complex::new(1.0, 2.0) * 2.0;
        assert_eq!(a, Complex::new(2.0, 4.0));

        a *= 2.0;
        assert_eq!(a, Complex::new(4.0, 8.0));
    }

    #[test]
    fn test_multiplication() {
        let mut c = Complex::new(3.0, 4.0) * Complex::new(3.0, 4.0);
        assert_eq!(c, Complex::new(-7.0, 24.0));

        c *= Complex::new(-2.0, -2.0);
        assert_eq!(c, Complex::new(62.0, -34.0));
    }

    #[test]
    fn test_division() {
        let mut c = Complex::new(100.0, 1000.0) / Complex::new(10.0, 10.0);
        assert_eq!(c, Complex::new(55.0, 45.0));

        c /= Complex::new(2.0, 2.0);
        assert_eq!(c, Complex::new(25.0, -2.5));
    }

    #[test]
    fn test_square_root() {
        for x in 0..100 {
            for y in 0..100 {
                let a = Complex::new(x as f32 / 10.0, y as f32 / 10.0);
                let sqrt = (a * a).sqrt();
                let comparison = sqrt - a;
                dbg!(a);
                dbg!(sqrt);
                dbg!(comparison);
                assert!(Abs::abs(comparison.real) <= f32::EPSILON);
                assert!(Abs::abs(comparison.imag) <= f32::EPSILON);
            }
        }
    }
}