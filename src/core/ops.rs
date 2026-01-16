use crate::core::Vector;
use crate::traits::{Field, MulAdd};
use std::ops;

pub fn lerp<T>(u: T, v: T, coeff: f32) -> T where T: ops::Mul<f32, Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Clone {
    (v - u.clone()) * coeff + u
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Matrix;

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