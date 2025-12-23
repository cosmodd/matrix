use std::fmt;
use crate::core::matrix::Matrix;

#[derive(Debug)]
pub struct Vector<K> {
    data: Matrix<K>,
}

impl<K> Vector<K>
where
    K: Clone,
{
    pub fn size(&self) -> usize {
        self.data.shape().1
    }
}

impl<K> Clone for Vector<K>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Vector {
            data: self.data.clone(),
        }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Clone,
{
    fn from(value: [K; N]) -> Self {
        Vector {
            data: Matrix::from_columns([value]),
        }
    }
}

impl<K> fmt::Display for Vector<K> where K: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)?;
        Ok(())
    }
}
