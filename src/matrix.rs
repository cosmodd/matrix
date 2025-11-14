use std::fmt;

#[derive(Debug)]
pub enum MatrixError {
    SizeMismatch,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    data: Vec<Vec<f32>>
}

impl Matrix {
    fn col_count(&self) -> usize {
        self.data.len()
    }

    fn row_count(&self) -> usize {
        self.data.get(0).map_or(0, |row| row.len())
    }

    fn is_square(&self) -> bool {
        self.col_count() == self.row_count()
    }

    pub fn add(&mut self, v: &Matrix) -> Result<(), MatrixError> {
        if self.col_count() != v.col_count() || self.row_count() != v.row_count() {
            return Err(MatrixError::SizeMismatch);
        }

        for x in 0..self.col_count() {
            for y in 0..v.row_count() {
                self.data[x][y] = self.data[x][y] + v.data[x][y];
            }
        }

        Ok(())
    }

    pub fn sub(&mut self, v: &Matrix) -> Result<(), MatrixError> {
        if self.col_count() != v.col_count() || self.row_count() != v.row_count() {
            return Err(MatrixError::SizeMismatch);
        }

        for x in 0..self.col_count() {
            for y in 0..v.row_count() {
                self.data[x][y] = self.data[x][y] - v.data[x][y];
            }
        }

        Ok(())
    }

    pub fn scl(&mut self, scalar: f32) {
        for x in 0..self.col_count() {
            for y in 0..self.row_count() {
                self.data[x][y] = self.data[x][y] * scalar;
            }
        }
    }
}

impl<const R: usize, const C: usize> From<[[f32; R]; C]> for Matrix {
    fn from(value: [[f32; R]; C]) -> Self {
        let rows = value.len();
        let cols = value[0].len();

        let mut columns = Vec::with_capacity(cols);

        for i in 0..cols {
            let mut column = Vec::with_capacity(rows);
            for j in 0..rows {
                column.push(value[j][i].clone());
            }

            columns.push(column);
        }

        Matrix { data: columns }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return write!(f, "┌ ┐\n└ ┘");
        }

        let num_cols = self.col_count();
        let num_rows = self.row_count();

        if num_rows == 0 {
            return write!(f, "┌ ┐\n└ ┘");
        }

        let mut rows = vec![Vec::new(); num_rows];
        let mut max_widths = vec![0; num_cols];

        for (x, col) in self.data.iter().enumerate() {
            for (y, value) in col.iter().enumerate() {
                let formatted = format!("{:.2}", value);
                max_widths[x] = max_widths[x].max(formatted.len());
                rows[y].push(formatted);
            }
        }

        let total_width = max_widths.iter().sum::<usize>() + (num_cols - 1) * 2;

        writeln!(f, "┌ {} ┐", " ".repeat(total_width))?;

        for row in rows {
            writeln!(f, "│ {} │", row.iter().enumerate().map(|(i, v)| format!("{:>width$}", v, width = max_widths[i])).collect::<Vec<String>>().join("  "))?;
        }

        writeln!(f, "└ {} ┘", " ".repeat(total_width))?;

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_from() {
        let mat = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);

        assert_eq!(mat.col_count(), 3);
        assert_eq!(mat.row_count(), 3);
        assert_eq!(mat.is_square(), true);

        let mat = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);

        assert_eq!(mat.col_count(), 3);
        assert_eq!(mat.row_count(), 2);
        assert_eq!(mat.is_square(), false);
    }

    #[test]
    fn test_matrix_add() -> Result<(), MatrixError> {
        let mut a = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        let b = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);

        a.add(&b)?;

        assert_eq!(a, Matrix::from([
            [2.0, 4.0, 6.0],
            [8.0, 10.0, 12.0],
            [14.0, 16.0, 18.0],
        ]));
        Ok(())
    }

    #[test]
    fn test_matrix_sub() -> Result<(), MatrixError> {
        let mut a = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        let b = Matrix::from([
            [9.0, 8.0, 7.0],
            [6.0, 5.0, 4.0],
            [3.0, 2.0, 1.0],
        ]);

        a.sub(&b)?;

        assert_eq!(a, Matrix::from([
            [-8., -6., -4.],
            [-2., 0., 2.],
            [4., 6., 8.]
        ]));
        Ok(())
    }

    #[test]
    fn test_matrix_scalar() -> Result<(), MatrixError> {
        let mut a = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        a.scl(10.0);
        assert_eq!(a, Matrix::from([
            [10.0, 20.0, 30.0],
            [40.0, 50.0, 60.0],
            [70.0, 80.0, 90.0],
        ]));
        Ok(())
    }
}