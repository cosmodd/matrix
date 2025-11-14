use crate::vector::Vector;

pub fn linear_combination(u: &[Vector], coefs: &[f32]) -> Result<Vector, String> {
    if u.len() != coefs.len() {
        return Err("u.len() != coefs.len()".to_string());
    }

    let mut us = u.to_owned();

    us.iter_mut().enumerate().for_each(|(i, u)| {
        (*u).scl(coefs[i]);
    });

    let (first, rest) = us.split_at_mut(1);
    for i in 0..(rest.len()) {
        let _ = first[0].add(&rest[i]);
    }

    Ok(us[0].clone())
}