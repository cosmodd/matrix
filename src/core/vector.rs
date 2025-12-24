#![allow(dead_code)]

use crate::core::matrix::Matrix;
use crate::traits::Field;
use std::fmt;

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
