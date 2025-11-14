use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum VectorError {
    SizeMismatch,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vector {
    data: Vec<f32>
}

impl Vector {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn add(&mut self, v: &Vector) -> Result<(), VectorError> {
        if self.size() != v.size() {
            return Err(VectorError::SizeMismatch);
        }

        self.data.iter_mut().zip(v.data.iter()).for_each(|(x, y)| *x = *x + *y);
        Ok(())
    }

    pub fn sub(&mut self, v: &Vector) -> Result<(), VectorError> {
        if self.size() != v.size() {
            return Err(VectorError::SizeMismatch);
        }

        self.data.iter_mut().zip(v.data.iter()).for_each(|(x, y)| *x = *x - *y);
        Ok(())
    }

    pub fn scl(&mut self, scalar: f32) {
        self.data.iter_mut().for_each(|v| *v = *v * scalar);
    }
}

impl<const N: usize> From<[f32; N]> for Vector {
    fn from(values: [f32; N]) -> Self {
        Self { data: values.to_vec() }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..(self.data.len() - 1) {
            write!(f, "{} ", self.data[i])?;
        }
        write!(f, "{}", self.data[self.data.len() - 1])?;
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() -> Result<(), VectorError> {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add(&v)?;
        assert_eq!(u, Vector::from([7., 10.]), "testing addition of {} + {}", u, v);
        Ok(())
    }

    #[test]
    fn vector_subtraction() -> Result<(), VectorError> {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.sub(&v)?;
        assert_eq!(u, Vector::from([-3., -4.]), "testing subtraction of {} - {}", u, v);
        Ok(())
    }

    #[test]
    fn vector_scalar_multiplication() -> Result<(), VectorError> {
        let mut u = Vector::from([2., 3.]);
        let scalar = 5.;
        u.scl(scalar);
        assert_eq!(u, Vector::from([10., 15.]), "testing scalar multiplication of {} * {}", u, scalar);
        Ok(())
    }
}