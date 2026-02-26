#![allow(dead_code)]

use crate::core::Matrix;
use crate::traits::{Field, Trig};
use std::ops;

pub fn lerp<T>(u: T, v: T, coeff: f32) -> T where T: ops::Mul<f32, Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Clone {
    (v - u.clone()) * coeff + u
}

pub fn perspective_projection<T>(fov: T, aspect: T, near: T, far: T) -> Matrix<T>
where
    T: Field + Trig + From<f32>
{
    let mut mat = Matrix::<T>::from_elem(T::zero(), 4, 4);
    let inverse_tan_fov = T::one() / (fov / 2.0.into()).to_radians().tan();
    let inverse_aspect = T::one() / aspect;

    mat[(0, 0)] = inverse_aspect * inverse_tan_fov;
    mat[(1, 1)] = inverse_tan_fov;
    mat[(2, 2)] = -((far + near) / (far - near));
    mat[(3, 2)] = -((<f32 as Into<T>>::into(2.0) * far * near) / (far - near));
    mat[(2, 3)] = -T::one();

    mat
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Vector;

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