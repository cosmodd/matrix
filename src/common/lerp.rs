use std::ops;

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: ops::Add<Output = V> + ops::Sub<Output = V> + ops::Mul<f32, Output = V> + Clone,
{
    u.clone() + (v.clone() - u.clone()) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0., 1., 0.), 0.);
        assert_eq!(lerp(0., 1., 1.), 1.);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);

        let v1 = Vector::from([2., 1.]);
        let v2 = Vector::from([4., 2.]);
        assert_eq!(lerp(v1, v2, 0.3), Vector::from([2.6, 1.3]));

        let m1 = Matrix::from([
            [2., 1.],
            [3., 4.]
        ]);
        let m2 = Matrix::from([
            [20., 10.],
            [30., 40.]
        ]);
        assert_eq!(lerp(m1, m2, 0.5), Matrix::from([
            [11., 5.5],
            [16.5, 22.]
        ]));
    }
}