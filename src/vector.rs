use crate::field::Field;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Vector<K> {
    data: Vec<K>
}

impl<K: Field> Vector<K> {
    pub fn new(data: Vec<K>) -> Vector<K> {
        Vector { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K: Field> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl<K: Field, const S: usize> From<[K; S]> for Vector<K> {
    fn from(data: [K; S]) -> Self {
        Vector { data: data.to_vec() }
    }
}

impl<K: Field> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        let numbers = self.data.iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<String>>()
            .join("  ");
        write!(f, "{}", numbers)?;
        write!(f, " )")
    }
}