use crate::core::Vector;
use crate::traits;
use crate::traits::Field;
use std::ops;

pub fn linear_combination<K: Field>(vectors: &[Vector<K>], coeffs: &[K]) -> Vector<K> {
    assert!(!vectors.is_empty(), "vectors array must not be empty");
    assert_eq!(vectors.len(), coeffs.len(), "vectors array and coeffs array must have the same length");

    let vector_size = vectors[0].size();
    assert!(vectors.iter().all(|v| v.size() == vector_size), "vectors array must have same size");

    let mut result: Vector<K> = Vector::from_elem(K::zero(), vector_size);

    for (vector, coeff) in vectors.iter().zip(coeffs.iter()) {
        for i in 0..vector.size() {
            result[i] = traits::MulAdd::mul_add(vector[i], *coeff, result[i]);
        }
    }

    result
}

pub fn lerp<T>(u: T, v: T, coeff: f32) -> T where T: ops::Mul<f32, Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Clone {
    (v - u.clone()) * coeff + u
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Matrix;

    #[test]
    fn test_linear_combination() {
        let e1: Vector<f32> = Vector::from([1., 0., 0.]);
        let e2: Vector<f32> = Vector::from([0., 1., 0.]);
        let e3: Vector<f32> = Vector::from([0., 0., 1.]);
        assert_eq!(linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), Vector::from([10., -2., 0.5]));

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert_eq!(linear_combination(&[v1, v2], &[10., -2.]), Vector::from([10., 0., 230.]));
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0., 1., 0.), 0.);
        assert_eq!(lerp(0., 1., 1.), 1.);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
        assert_eq!(lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3), Vector::from([2.6, 1.3]));

        let a = Matrix::from_rows([
            [2., 1.],
            [3., 4.]
        ]);
        let b = Matrix::from_rows([
            [20., 10.],
            [30., 40.]
        ]);
        assert_eq!(lerp(a.clone(), b.clone(), 0.5), Matrix::from_rows([
            [11., 5.5],
            [16.5, 22.]
        ]));
    }
}