use crate::traits::Field;
use std::{fmt, ops};

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

impl<K: Field> PartialEq for Matrix<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.rows() != other.rows() || self.columns() != other.columns() {
            return false;
        }

        for row in 0..self.rows() {
            for column in 0..self.columns() {
                if self.data[column][row] != other.data[column][row] {
                    return false;
                }
            }
        }

        true
    }
}

impl<K: Field, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        let mut columns = vec![vec![K::zero(); R]; C];

        for column in 0..C {
            for row in 0..R {
                columns[column][row] = data[row][column].clone();
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

impl<K: Field> ops::Add for Matrix<K> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        if self.rows() != other.rows() || self.columns() != other.columns() {
            panic!("Matrices must have the same sizes");
        }

        for x in 0..self.columns() {
            for y in 0..self.rows() {
                self.data[x][y] = self.data[x][y] + other.data[x][y];
            }
        }

        self
    }
}

impl<K: Field> ops::Sub for Matrix<K> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        if self.rows() != other.rows() || self.columns() != other.columns() {
            panic!("Matrices must have the same sizes");
        }

        for x in 0..self.columns() {
            for y in 0..self.rows() {
                self.data[x][y] = self.data[x][y] - other.data[x][y];
            }
        }

        self
    }
}

impl<K: Field> ops::Mul<K> for Matrix<K> {
    type Output = Self;

    fn mul(mut self, other: K) -> Self::Output {
        for x in 0..self.columns() {
            for y in 0..self.rows() {
                self.data[x][y] = self.data[x][y] * other;
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn matrix_equality() {
        let a = Matrix::from([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
        ]);
        let b = a.clone();

        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_ne!(a, Matrix::zeros(1, 1));
    }

    #[test]
    fn matrix_addition() {
        let a = Matrix::from([
            [1., 2.],
            [3., 4.],
        ]);
        let b = Matrix::from([
            [7., 4.],
            [-2., 2.]
        ]);

        assert_eq!(a + b, Matrix::from([
            [8., 6.],
            [1., 6.]
        ]));
    }

    #[test]
    fn matrix_subtraction() {
        let a = Matrix::from([
            [1., 2.],
            [3., 4.],
        ]);
        let b = Matrix::from([
            [7., 4.],
            [-2., 2.]
        ]);

        assert_eq!(a - b, Matrix::from([
            [-6., -2.],
            [5., 2.]
        ]));
    }

    #[test]
    fn matrix_scalar_mul() {
        let a = Matrix::from([
            [1., 2.],
            [3., 4.],
        ]);

        assert_eq!(a.clone() * 0., Matrix::zeros(2, 2));
        assert_eq!(a.clone() * 2., Matrix::from([
            [2., 4.],
            [6., 8.]
        ]));
    }
}