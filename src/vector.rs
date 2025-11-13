use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Debug)]
pub enum VectorError {
    SizeMismatch,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Vector<K> {
    data: Vec<K>
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K> Vector<K> where K: ops::Add<Output=K> + Copy
{
    pub fn add(&mut self, v: &Vector<K>) -> Result<(), VectorError> {
        if self.size() != v.size() {
            return Err(VectorError::SizeMismatch);
        }

        self.data.iter_mut().zip(v.data.iter()).for_each(|(x, y)| *x = *x + *y);
        Ok(())
    }
}

impl<K> Vector<K> where K: ops::Sub<Output=K> + Copy {
    pub fn sub(&mut self, v: &Vector<K>) -> Result<(), VectorError> {
        if self.size() != v.size() {
            return Err(VectorError::SizeMismatch);
        }

        self.data.iter_mut().zip(v.data.iter()).for_each(|(x, y)| *x = *x - *y);
        Ok(())
    }
}

impl<K> Vector<K> where K: ops::Mul<Output=K> + Copy {
    pub fn scl(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|v| *v = *v * scalar);
    }
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
    fn from(values: [K; N]) -> Self {
        Self { data: values.to_vec() }
    }
}

impl<K> Display for Vector<K> where K: Display {
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