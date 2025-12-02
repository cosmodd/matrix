use crate::field::Field;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    data: Vec<Vec<K>>
}

impl<K: Field> Matrix<K> {
    pub fn zeros(rows: usize, cols: usize) -> Matrix<K> {
        Matrix { data: vec![vec![K::zero(); rows]; cols] }
    }

    pub fn identity(size: usize) -> Matrix<K> {
        let mut base = Matrix::zeros(size, size);

        for column in 0..size {
            for row in 0..size {
                if row == column {
                    base.data[column][row] = K::one();
                }
            }
        }

        base
    }

    pub fn columns(&self) -> usize {
        self.data.len()
    }

    pub fn rows(&self) -> usize {
        self.data[0].len()
    }

    pub fn is_square(&self) -> bool {
        self.rows() == self.columns()
    }
}

impl<K: Field, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        let mut columns = vec![vec![K::zero(); R]; C];

        for column in 0..C {
            for row in 0..R {
                columns[row][column] = data[column][row].clone();
            }
        }

        Matrix { data: columns }
    }
}

impl<K: Field> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows: usize = self.rows();
        let columns: usize = self.columns();
        let gap: usize  = 2;

        let max_lengths: Vec<usize> = self.data.iter()
            .map(|column| column.iter().map(|val| val.to_string().len()).max().unwrap_or(0))
            .collect();

        let first_last_spaces = " ".repeat(max_lengths.iter().sum::<usize>() + gap * (columns - 1) + 2);

        writeln!(f, "┌{}┐", first_last_spaces)?;
        for row in 0..rows {
            write!(f, "│ ")?;
            for column in 0..columns {
                let max_length = max_lengths[column];

                write!(f, "{:size$}", self.data[column][row], size = max_length)?;
                if (column + 1) < columns {
                    write!(f, "{:sep$}", " ", sep=gap)?;
                }
            }
            writeln!(f, " │")?;
        }
        write!(f, "└{}┘", first_last_spaces)
    }
}