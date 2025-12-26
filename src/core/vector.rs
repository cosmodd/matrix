#![allow(dead_code)]

use crate::core::matrix::Matrix;
use crate::traits::Field;
use std::{fmt, ops};

#[derive(Debug)]
pub struct Vector<K: Field> {
    data: Matrix<K>,
}

impl<K: Field> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.shape().1
    }
}

impl<K: Field> Clone for Vector<K> {
    fn clone(&self) -> Self {
        Vector {
            data: self.data.clone(),
        }
    }
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K> {
    fn from(value: [K; N]) -> Self {
        Vector {
            data: Matrix::from_columns([value]),
        }
    }
}

impl<K: Field> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)?;
        Ok(())
    }
}

impl<K: Field> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }

        self.data == other.data
    }
}

impl<K: Field> ops::Add for Vector<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "Vector addition size mismatch.");
        Vector {
            data: self.data + rhs.data,
        }
    }
}

impl<K: Field> ops::Sub for Vector<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "Vector subtraction size mismatch.");
        Vector {
            data: self.data - rhs.data,
        }
    }
}

impl<K: Field> ops::Mul<K> for Vector<K> {
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        Vector {
            data: self.data * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let a = Vector::from([1., 2., 3.]);
        let b = Vector::from([4., 5., 6.]);
        assert_eq!(a + b, Vector::from([5., 7., 9.]));
    }

    #[test]
    #[should_panic]
    fn test_vector_addition_panic() {
        let a = Vector::from([1., 2., 3.]);
        let b = Vector::from([4., 5., 6., 7.]);
        let _ = a + b;
    }
    #[test]
    fn test_vector_subtraction() {
        let a = Vector::from([1., 2., 3.]);
        let b = Vector::from([4., 5., 6.]);
        assert_eq!(a - b, Vector::from([-3., -3., -3.]));
    }

    #[test]
    #[should_panic]
    fn test_vector_substraction_panic() {
        let a = Vector::from([1., 2., 3.]);
        let b = Vector::from([4., 5., 6., 7.]);
        let _ = a - b;
    }

    #[test]
    fn test_vector_multiplication() {
        let a = Vector::from([1., 2., 3.]);
        assert_eq!(a.clone() * 2., Vector::from([2., 4., 6.]));
        assert_eq!(a.clone() * -2., Vector::from([-2., -4., -6.]));
    }

}
