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