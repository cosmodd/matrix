#![allow(dead_code)]

use crate::core::matrix::Matrix;
use crate::traits::{Abs, Field, MulAdd, Sqrt};
use std::{fmt, ops};

#[derive(Debug)]
pub struct Vector<K: Field> {
    data: Matrix<K>,
}

impl<K: Field> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.shape().1
    }

    pub fn from_elem(elem: K, size: usize) -> Self {
        Vector {
            data: Matrix::from_elem(elem, 1, size)
        }
    }

    pub fn linear_combination(vectors: &[Vector<K>], coeffs: &[K]) -> Vector<K> {
        assert!(!vectors.is_empty(), "vectors array must not be empty");
        assert_eq!(vectors.len(), coeffs.len(), "vectors array and coeffs array must have the same length");

        let vector_size = vectors[0].size();
        assert!(vectors.iter().all(|v| v.size() == vector_size), "vectors array must have same size");

        let mut result: Vector<K> = Vector::from_elem(K::zero(), vector_size);

        for (vector, coeff) in vectors.iter().zip(coeffs.iter()) {
            for i in 0..vector.size() {
                result[i] = MulAdd::mul_add(vector[i], *coeff, result[i]);
            }
        }

        result
    }

    pub fn dot(&self, other: &Vector<K>) -> K {
        assert_eq!(self.size(), other.size(), "Vector dot size mismatch.");
        let mut result = K::zero();

        for i in 0..self.size() {
            result = MulAdd::mul_add(self[i], other[i], result);
        }

        result
    }

    pub fn norm_1(&self) -> K {
        let mut result: K = K::zero();

        for i in 0..self.size() {
            result = result + Abs::abs(self[i]);
        }

        result
    }

    pub fn norm(&self) -> K {
        let mut result: K = K::zero();

        for i in 0..self.size() {
            result = result + (self[i] * self[i])
        }

        Sqrt::sqrt(result)
    }

    pub fn norm_inf(&self) -> K {
        let mut result = Abs::abs(self[0]);

        for i in 1..self.size() {
            let abs_value = Abs::abs(self[i]);
            if abs_value > result {
                result = abs_value;
            }
        }

        result
    }

    pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
        u.dot(&v) / (u.norm() * v.norm())
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

impl<K: Field> ops::Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size());
        &self.data[(0, index)]
    }
}

impl<K: Field> ops::IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size());
        &mut self.data[(0, index)]
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
    fn test_vector_index() {
        let vec = Vector::from([1., 2., 3.]);
        assert_eq!(vec[0], 1.);
        assert_eq!(vec[1], 2.);
        assert_eq!(vec[2], 3.);
    }

    #[test]
    fn test_vector_index_mut() {
        let mut vec = Vector::from([1., 2., 3.]);
        assert_eq!(vec[0], 1.);
        assert_eq!(vec[1], 2.);
        assert_eq!(vec[2], 3.);

        vec[0] = 3.;
        vec[1] = 4.;
        vec[2] = 5.;

        assert_eq!(vec[0], 3.);
        assert_eq!(vec[1], 4.);
        assert_eq!(vec[2], 5.);
    }

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

    #[test]
    fn test_linear_combination() {
        let e1: Vector<f32> = Vector::from([1., 0., 0.]);
        let e2: Vector<f32> = Vector::from([0., 1., 0.]);
        let e3: Vector<f32> = Vector::from([0., 0., 1.]);
        assert_eq!(Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), Vector::from([10., -2., 0.5]));

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert_eq!(Vector::linear_combination(&[v1, v2], &[10., -2.]), Vector::from([10., 0., 230.]));
    }

    #[test]
    fn test_vector_dot() {
        assert_eq!(Vector::from([0., 0.]).dot(&Vector::from([1., 1.])), 0.);
        assert_eq!(Vector::from([1., 1.]).dot(&Vector::from([1., 1.])), 2.);
        assert_eq!(Vector::from([-1., 6.]).dot(&Vector::from([3., 2.])), 9.);
    }

    #[test]
    fn test_vector_norm_1() {
        let u = Vector::from_elem(0., 3);
        assert_eq!(u.norm_1(), 0.);

        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);

        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
    }

    #[test]
    fn test_vector_norm() {
        let u: Vector<f32> = Vector::from_elem(0., 3);
        assert_eq!(u.norm(), 0.);

        let u: Vector<f32> = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm(), 3.74165738);

        let u: Vector<f32> = Vector::from([-1., -2.]);
        assert_eq!(u.norm(), 2.236067977);
    }

    #[test]
    fn test_vector_norm_inf() {
        let u: Vector<f32> = Vector::from_elem(0., 3);
        assert_eq!(u.norm_inf(), 0.);

        let u: Vector<f32> = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_inf(), 3.0);

        let u: Vector<f32> = Vector::from([-1., -2.]);
        assert_eq!(u.norm_inf(), 2.0);
    }

    #[test]
    fn test_angle_cos() {
        let u: Vector<f32> = Vector::from([1., 0.]);
        let v: Vector<f32> = Vector::from([1., 0.]);
        assert_eq!(Vector::angle_cos(&u, &v), 1.);

        let u: Vector<f32> = Vector::from([1., 0.]);
        let v: Vector<f32> = Vector::from([0., 1.]);
        assert_eq!(Vector::angle_cos(&u, &v), 0.);

        let u: Vector<f32> = Vector::from([-1., 1.]);
        let v: Vector<f32> = Vector::from([1., -1.]);
        assert!(Vector::angle_cos(&u, &v) + 1. <= f32::EPSILON);

        let u: Vector<f32> = Vector::from([2., 1.]);
        let v: Vector<f32> = Vector::from([4., 2.]);
        assert!(Vector::angle_cos(&u, &v) - 1. <= f32::EPSILON);
        let u: Vector<f64> = Vector::from([1., 0.]);

        let v: Vector<f64> = Vector::from([1., 0.]);
        assert_eq!(Vector::angle_cos(&u, &v), 1.);

        let u: Vector<f64> = Vector::from([1., 0.]);
        let v: Vector<f64> = Vector::from([0., 1.]);
        assert_eq!(Vector::angle_cos(&u, &v), 0.);

        let u: Vector<f64> = Vector::from([-1., 1.]);
        let v: Vector<f64> = Vector::from([1., -1.]);
        assert!(Vector::angle_cos(&u, &v) + 1. <= f64::EPSILON);

        let u: Vector<f64> = Vector::from([2., 1.]);
        let v: Vector<f64> = Vector::from([4., 2.]);
        assert!(Vector::angle_cos(&u, &v) - 1. <= f64::EPSILON);
    }
}
