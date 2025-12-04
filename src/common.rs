use crate::field::Field;
use crate::vector::Vector;

pub fn linear_combination<K: Field>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    if u.len() != coefs.len() {
        panic!("u and coefs length do not match");
    }

    let mut result = Vector::zeros(u[0].size());

    for i in 0..coefs.len() {
        let vec = u[i].clone();
        let coef = coefs[i];

        result = Vector::mul_add(vec, &result, coef);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn test_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        assert_eq!(linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), Vector::from([10., -2., 0.5]));

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert_eq!(linear_combination(&[v1, v2], &[10., -2.]), Vector::from([10., 0., 230.]));
    }
}