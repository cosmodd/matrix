use crate::matrix::{Matrix, MatrixError};
use crate::vector::{Vector, VectorError};

pub fn linear_combination(u: &[Vector], coefs: &[f32]) -> Result<Vector, String> {
    if u.is_empty() {
        return Err(String::from("Empty vector array"));
    }

    if u.len() != coefs.len() {
        return Err("u.len() != coefs.len()".to_string());
    }

    let mut result = u[0].clone();
    result.scl(coefs[0]);

    for i in 1..u.len() {
        let mut scaled_vector = u[i].clone();
        scaled_vector.scl(coefs[i]);
        result.add(&scaled_vector).map_err(|e| "Vector size mismatch".to_string())?;
    }

    Ok(result)
}

pub fn lerp(u: f32, v: f32, t: f32) -> f32 {
    u + (v - u) * t
}

pub fn lerp_vector(u: &Vector, v: &Vector, t: f32) -> Result<Vector, VectorError> {
    let mut result = v.clone();
    result.sub(&u)?;
    result.scl(t);
    result.add(&u)?;
    Ok(result)
}

pub fn lerp_matrix(u: &Matrix, v: &Matrix, t: f32) -> Result<Matrix, MatrixError> {
    let mut result = v.clone();
    result.sub(&u)?;
    result.scl(t);
    result.add(&u)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_combination() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        let lc = linear_combination(&[v1, v2], &[10., -2.]).unwrap();

        assert_eq!(lc, Vector::from([10., 0., 230.]));
    }
}