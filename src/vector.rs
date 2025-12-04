use crate::traits::Field;
use std::fmt::Formatter;
use std::{fmt, ops};

#[derive(Debug, Clone)]
pub struct Vector<K> {
    data: Vec<K>
}

impl<K: Field> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn zeros(size: usize) -> Vector<K> {
        Self { data: vec![K::zero(); size] }
    }

    pub fn mul_add(mut self, other: &Self, scalar: K) -> Self {
        for i in 0..self.size() {
            self.data[i] = self.data[i].mul_add(scalar, other.data[i]);
        }

        self
    }
}

impl<K: Field> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl<K: Field, const S: usize> From<[K; S]> for Vector<K> {
    fn from(data: [K; S]) -> Self {
        Vector { data: data.to_vec() }
    }
}

impl<K: Field> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        let numbers = self.data.iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<String>>()
            .join("  ");
        write!(f, "{}", numbers)?;
        write!(f, " )")
    }
}

impl<K: Field> ops::Add for Vector<K> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        if self.size() != other.size() {
            panic!("Vectors must have the same size!");
        }

        for (i, value) in self.data.iter_mut().enumerate() {
            *value = *value + other.data[i];
        }
        self
    }
}

impl<K: Field> ops::Sub for Vector<K> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        if self.size() != other.size() {
            panic!("Vectors must have the same size!");
        }

        for (i, value) in self.data.iter_mut().enumerate() {
            *value = *value - other.data[i];
        }
        self
    }
}

impl<K: Field> ops::Mul<K> for Vector<K> {
    type Output = Self;

    fn mul(mut self, other: K) -> Self::Output {
        for  value in self.data.iter_mut() {
            *value = *value * other;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_add() {
        let a = Vector::from([1., 2., 3.]);
        let b = Vector::from([3., 2., 1.]);

        assert_eq!(a.clone() + a.clone(), Vector::from([2., 4., 6.]));
        assert_eq!(b.clone() + Vector::zeros(3), b);
    }

    #[test]
    fn vector_sub() {
        let a = Vector::from([1., 2., 3.]);
        let b = Vector::from([3., 2., 1.]);
        assert_eq!(a.clone() - a.clone(), Vector::zeros(3));
        assert_eq!(Vector::zeros(3) - b.clone(), Vector::from([-3., -2., -1.]));
    }

    #[test]
    fn vector_scalar_mul() {
        let a = Vector::from([1., 2., 3.]);
        assert_eq!(a.clone() * 2., Vector::from([2., 4., 6.]));
        assert_eq!(a.clone() * -1., Vector::from([-1., -2., -3.]));
    }
}