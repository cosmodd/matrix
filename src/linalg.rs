use crate::vector::Vector;

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