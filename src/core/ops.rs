use crate::core::Vector;
use crate::traits;
use crate::traits::Field;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}