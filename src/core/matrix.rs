#![allow(dead_code)]

use crate::core::Vector;
use crate::traits::{Abs, Field, MulAdd};
use std::{fmt, ops};

#[derive(Debug)]
pub struct Matrix<K: Field> {
    shape: (usize, usize),
    data: Vec<K>,
}

impl<K: Field> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn is_square(&self) -> bool {
        self.shape.0 == self.shape.1
    }

    pub fn from_rows<const W: usize, const H: usize>(values: [[K; W]; H]) -> Self {
        let mut data = Vec::<K>::with_capacity(W * H);

        for x in 0..W {
            for y in 0..H {
                data.push(values[y][x]);
            }
        }

        Matrix {
            shape: (W, H),
            data,
        }
    }

    pub fn from_columns<const W: usize, const H: usize>(values: [[K; H]; W]) -> Self {
        let mut data = Vec::<K>::with_capacity(W * H);

        for x in 0..W {
            for y in 0..H {
                data.push(values[x][y]);
            }
        }

        Matrix {
            shape: (W, H),
            data,
        }
    }
    
    pub fn from_elem(elem: K, width: usize, height: usize) -> Self {
        Matrix {
            shape: (width, height),
            data: vec![elem; width * height],
        }
    }

    pub fn trace(&self) -> K {
        assert!(self.is_square(), "Trace is only defined for square matrices");
        let mut result = K::zero();

        for i in 0..self.shape.0 {
            result = result + self[(i, i)];
        }

        result
    }

    pub fn transpose(&self) -> Matrix<K> {
        let mut result = Matrix::from_elem(K::zero(), self.shape.1, self.shape.0);

        for y in 0..self.shape.1 {
            for x in 0..self.shape.0 {
                result[(y, x)] = self[(x, y)];
            }
        }

        result
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let mut result = self.clone();
        let (cols, rows) = result.shape;
        let mut y: usize = 0;

        for x in 0..cols {
            let pivot_row = (y..rows).find(|&y| { result[(x, y)] != K::zero() });
            if pivot_row.is_none() {
                continue;
            }
            let pivot_row = pivot_row.unwrap();

            // Swap current row with pivot row
            if pivot_row != y {
                for i in 0..cols {
                    let temp = result[(i, y)];
                    result[(i, y)] = result[(i, pivot_row)];
                    result[(i, pivot_row)] = temp;
                }
            }

            // Normalize pivot (left-most number should be one)
            let pivot = result[(x, y)];
            for i in 0..cols {
                result[(i, y)] = result[(i, y)] / pivot;
            }

            // Simplifying other lines
            for row in 0..rows {
                if row == y { continue; }
                if result[(x, row)] == K::zero() { continue; }
                let mult = result[(x, row)] / result[(x, y)];
                for col in 0..cols {
                    result[(col, row)] = result[(col, row)] - result[(col, y)] * mult;
                }
            }

            y += 1;
        }

        result
    }

    pub fn determinant(&self) -> K {
        assert!(self.is_square(), "Determinant is only defined for square matrices");

        match self.shape.0 {
            1 => self.data[0],
            2 => self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)],
            size if size <= 4 => {
                let mut det_matrix = self.clone();
                let (cols, rows) = det_matrix.shape;
                let mut y: usize = 0;

                for x in 0..cols {
                    let pivot_row = (y..rows).find(|&y| { det_matrix[(x, y)] != K::zero() });
                    if pivot_row.is_none() {
                        continue;
                    }
                    let pivot_row = pivot_row.unwrap();

                    // R_y <=> -R_pr
                    if pivot_row != y {
                        for i in 0..cols {
                            let temp = -det_matrix[(i, y)];
                            det_matrix[(i, y)] = det_matrix[(i, pivot_row)];
                            det_matrix[(i, pivot_row)] = temp;
                        }
                    }

                    for row in (y + 1)..rows {
                        if det_matrix[(x, row)] == K::zero() { continue; }
                        let mult = det_matrix[(x, row)] / det_matrix[(x, y)];
                        for col in 0..cols {
                            det_matrix[(col, row)] = det_matrix[(col, row)] - det_matrix[(col, y)] * mult;
                        }
                    }

                    y += 1;
                }

                (0..rows).fold(K::one(), |acc, i| acc * det_matrix[(i, i)])
            },
            _ => K::zero(),
        }
    }
}

impl<K: Field> Clone for Matrix<K>
{
    fn clone(&self) -> Self {
        Matrix {
            shape: self.shape,
            data: self.data.clone(),
        }
    }
}

impl<K: Field> fmt::Display for Matrix<K>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = self.shape;

        let mut col_widths = vec![0; width];
        for col in 0..width {
            for row in 0..height {
                let index = col * height + row;
                let length = self.data[index].to_string().len();
                col_widths[col] = col_widths[col].max(length);
            }
        }

        for row in 0..height {
            let delim_chars = match row {
                0 => ('⎡', '⎤'),
                r if r == height - 1 => ('⎣', '⎦'),
                _ => ('⎢', '⎢'),
            };

            write!(f, "{} ", delim_chars.0)?;
            for col in 0..width {
                let index = col * height + row;
                let value = self.data[index].to_string();
                write!(f, "{:>width$}", value, width = col_widths[col])?;
                if col < width - 1 {
                    write!(f, "  ")?;
                }
            }
            write!(f, " {}", delim_chars.1)?;
            if row < height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<K: Field> PartialEq for Matrix<K>
{
    fn eq(&self, other: &Self) -> bool {
        if self.shape != other.shape {
            return false;
        }

        let (width, height) = self.shape;
        for x in 0..width {
            for y in 0..height {
                if self.data[x + y * width] != other.data[x + y * width] {
                    return false;
                }
            }
        }

        true
    }
}

impl<K: Field> ops::Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.shape.0);
        assert!(y < self.shape.1);
        &self.data[x * self.shape.1 + y]
    }
}

impl<K: Field> ops::IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.shape.0);
        assert!(y < self.shape.1);
        &mut self.data[x * self.shape.1 + y]
    }
}

impl<K: Field> ops::Add for Matrix<K> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "Matrix addition dimensions mismatch.");

        for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
            *a = *a + *b;
        }
        self
    }
}

impl<K: Field> ops::Sub for Matrix<K> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "Matrix substraction dimensions mismatch.");

        for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
            *a = *a - *b;
        }
        self
    }
}

impl<K: Field> ops::Mul<K> for Matrix<K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for value in self.data.iter_mut() {
            *value = *value * rhs
        }
        self
    }
}

impl<K: Field> ops::Mul<Matrix<K>> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, rhs: Matrix<K>) -> Self::Output {
        assert_eq!(self.shape().0, rhs.shape().1, "Left matrix width must equal right matrix height");
        let mut result = Matrix::from_elem(K::zero(), rhs.shape().0, self.shape().1);

        println!("======================================");
        println!("{self}");
        println!("{rhs}");
        for ly in 0..self.shape().1 {
            for rx in 0..rhs.shape().0 {
                for i in 0..self.shape().0 {
                    println!("[{rx}, {ly}; {}] += {} * {}", result[(rx, ly)], self[(i, ly)],  rhs[(rx, i)]);
                    result[(rx, ly)] = MulAdd::mul_add(self[(i, ly)], rhs[(rx, i)], result[(rx, ly)]);
                }
            }
        }

        result
    }
}

impl<K: Field> ops::Mul<Vector<K>> for Matrix<K> {
    type Output = Vector<K>;

    fn mul(self, mut rhs: Vector<K>) -> Self::Output {
        assert_eq!(rhs.size(), self.shape().0, "Vector size must equal matrix width");
        let mut result = Vector::from_elem(K::zero(), self.shape().1);

        for row in 0..self.shape().1 {
            for i in 0..rhs.size() {
                result[row] = MulAdd::mul_add(self[(i, row)], rhs[i], result[row]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_from_rows() {
        let mat = Matrix::from_rows([
            [1., 2., 3., 4., 5.],
            [6., 7., 8., 9., 10.],
        ]);

        assert_eq!(mat.data, [1., 6., 2., 7., 3., 8., 4., 9., 5., 10.]);
        assert_eq!(mat.shape, (5, 2));
    }

    fn test_matrix_from_columns() {
        let mat = Matrix::from_columns([
            [1., 2., 3., 4., 5.],
            [6., 7., 8., 9., 10.],
        ]);

        assert_eq!(mat.data, [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
        assert_eq!(mat.shape, (2, 5));
    }

    #[test]
    fn test_matrix_index() {
        let mat = Matrix::from_rows([
            [1., 2., 3., 4., 5.],
            [6., 7., 8., 9., 10.],
        ]);

        for i in 0..10 {
            let x = i % 5;
            let y = i / 5;
            assert_eq!(mat[(x, y)], (i + 1) as f64);
        }
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut mat = Matrix::from_rows([
            [1., 2., 3., 4., 5.],
            [6., 7., 8., 9., 10.],
        ]);

        for i in 0..10 {
            let x = i % 5;
            let y = i / 5;
            assert_eq!(mat[(x, y)], (i + 1) as f64);
        }

        for i in 0..10 {
            let x = i % 5;
            let y = i / 5;
            mat[(x, y)] = ((i as f64) + 1.) * 10.;
        }

        for i in 0..10 {
            let x = i % 5;
            let y = i / 5;
            assert_eq!(mat[(x, y)], ((i as f64) + 1.) * 10.);
        }
    }

    #[test]
    #[should_panic]
    fn test_matrix_index_panic() {
        let mat = Matrix::from_rows([
            [1., 2., 3., 4., 5.],
            [6., 7., 8., 9., 10.],
        ]);

        mat[(10, 10)];
    }

    #[test]
    fn test_matrix_addition() {
        let a = Matrix::from_rows([
            [1., 2., 3.],
            [4., 5., 6.],
            [7., 8., 9.],
        ]);
        let b = Matrix::from_rows([
            [1., 2., 2.],
            [3., 1., 2.],
            [3., 3., 1.],
        ]);

        assert_eq!(a + b, Matrix::from_rows([
            [2., 4., 5.],
            [7., 6., 8.],
            [10., 11., 10.],
        ]));
    }

    #[test]
    #[should_panic]
    fn test_matrix_addition_panic() {
        let a = Matrix::from_rows([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        let b = Matrix::from_rows([[1., 2.], [3., 4.], [5., 6.], [7., 8.]]);
        let _ = a + b;
    }

    #[test]
    fn test_matrix_subtraction() {
        let a = Matrix::from_rows([
            [1., 2., 3.],
            [4., 5., 6.],
            [7., 8., 9.],
        ]);
        let b = Matrix::from_rows([
            [1., 2., 2.],
            [3., 1., 2.],
            [3., 3., 1.],
        ]);

        assert_eq!(a - b, Matrix::from_rows([
            [0., 0., 1.],
            [1., 4., 4.],
            [4., 5., 8.],
        ]));
    }

    #[test]
    #[should_panic]
    fn test_matrix_substraction_panic() {
        let a = Matrix::from_rows([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        let b = Matrix::from_rows([[1., 2.], [3., 4.], [5., 6.], [7., 8.]]);
        let _ = a - b;
    }

    #[test]
    fn test_matrix_scalar() {
        let a = Matrix::from_rows([
            [1., 2., 3.],
            [4., 5., 6.],
            [7., 8., 9.],
        ]);

        assert_eq!(a.clone() * 10.0, Matrix::from_rows([
            [10., 20., 30.],
            [40., 50., 60.],
            [70., 80., 90.],
        ]));
        assert_eq!(a.clone() * -10.0, Matrix::from_rows([
            [-10., -20., -30.],
            [-40., -50., -60.],
            [-70., -80., -90.],
        ]));
    }

    #[test]
    fn test_matrix_mult_vec() {
        let a = Matrix::from_rows([
            [1., 0.],
            [0., 1.],
        ]);
        let u = Vector::from([4., 2.]);
        assert_eq!(a * u, Vector::from([4., 2.]));

        let a = Matrix::from_rows([
            [2., 0.],
            [0., 2.],
        ]);
        let u = Vector::from([4., 2.]);
        assert_eq!(a * u, Vector::from([8., 4.]));

        let a = Matrix::from_rows([
            [2., -2.],
            [-2., 2.],
        ]);
        let u = Vector::from([4., 2.]);
        assert_eq!(a * u, Vector::from([4., -4.]));

        let a = Matrix::from_rows([
            [5., 4., 3.],
            [8., 9., 5.],
            [6., 5., 3.],
            [11., 9., 6.]
        ]);
        let u = Vector::from([100., 80., 60.]);
        assert_eq!(a * u, Vector::from([1000., 1820., 1180., 2180.]));
    }

    #[test]
    fn test_matrix_mult_matrix() {
        let u = Matrix::from_rows([
            [1., 0.],
            [0., 1.],
        ]);
        let v = Matrix::from_rows([
             [1., 0.],
             [0., 1.],
         ]);
        assert_eq!(u * v, Matrix::from_rows([
            [1., 0.],
            [0., 1.]
        ]));

        let u = Matrix::from_rows([
            [1., 0.],
            [0., 1.],
        ]);
        let v = Matrix::from_rows([
            [2., 1.],
            [4., 2.],
        ]);
        assert_eq!(u * v, Matrix::from_rows([
            [2., 1.],
            [4., 2.]
        ]));

        let u = Matrix::from_rows([
            [3., -5.],
            [6., 8.],
        ]);
        let v = Matrix::from_rows([
            [2., 1.],
            [4., 2.],
        ]);
        assert_eq!(u * v, Matrix::from_rows([
            [-14., -7.],
            [44., 22.]
        ]));

        let u = Matrix::from_rows([
            [0., 4., -2.],
            [-4., -3., 0.],
        ]);
        let v = Matrix::from_rows([
            [0., 1.],
            [1., -1.],
            [2., 3.]
        ]);
        assert_eq!(u * v, Matrix::from_rows([
            [0., -10.],
            [-3., -1.]
        ]));
    }

    #[test]
    fn test_matrix_trace() {
        let u = Matrix::from_rows([
            [1., 0.],
            [0., 1.],
        ]);
        assert_eq!(u.trace(), 2.0);

        let u = Matrix::from_rows([
            [2., -5., 0.],
            [4., 3., 7.],
            [-2., 3., 4.],
        ]);
        assert_eq!(u.trace(), 9.0);

        let u = Matrix::from_rows([
            [-2., -8., 4.],
            [1., -23., 4.],
            [0., 6., 4.],
        ]);
        assert_eq!(u.trace(), -21.0);
    }

    #[test]
    fn test_matrix_transpose() {
        let u = Matrix::from_rows([[1., 2.]]);
        assert_eq!(u.transpose(), Matrix::from_columns([[1., 2.]]));

        let u = Matrix::from_rows([
            [1., 2.],
            [3., 4.],
        ]);
        assert_eq!(u.transpose(), Matrix::from_rows([
            [1., 3.],
            [2., 4.],
        ]));

        let u = Matrix::from_rows([
            [1., 2.],
            [3., 4.],
            [5., 6.],
        ]);
        assert_eq!(u.transpose(), Matrix::from_rows([
            [1., 3., 5.],
            [2., 4., 6.],
        ]));
    }

    #[test]
    pub fn test_row_echelon() {
        let u = Matrix::from_rows([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from_rows([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ]));

        let u = Matrix::from_rows([
            [1., 2.],
            [3., 4.],
        ]);
        println!("{}", u.row_echelon());
        assert_eq!(u.row_echelon(), Matrix::from_rows([
            [1.0, 0.0],
            [0.0, 1.0]
        ]));

        let u = Matrix::from_rows([
            [1., 2.],
            [2., 4.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from_rows([
            [1.0, 2.0],
            [0.0, 0.0]
        ]));

        let u = Matrix::from_rows([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from_rows([
            [1.0, 0.625, 0.0, 0.0, -12.166666666666668],
            [0.0, 0.0, 1.0, 0.0, -3.666666666666667],
            [0.0, 0.0, 0.0, 1.0, 29.500000000000004]
        ]));
    }

    #[test]
    pub fn test_determinant() {
        let u = Matrix::from_rows([
            [55.],
        ]);
        assert_eq!(u.determinant(), 55.0);

        let u = Matrix::from_rows([
            [ 1., -1.],
            [-1., 1.],
        ]);
        assert_eq!(u.determinant(), 0.0);

        let u = Matrix::from_rows([
            [2., 0., 0.],
            [0., 2., 0.],
            [0., 0., 2.],
        ]);
        assert_eq!(u.determinant(), 8.0);

        let u = Matrix::from_rows([
            [8., 5., -2.],
            [4., 7., 20.],
            [7., 6., 1.],
        ]);
        assert_eq!(u.determinant(), -174.0);

        let u = Matrix::from_rows([
            [ 8., 5., -2., 4.],
            [ 4., 2.5, 20., 4.],
            [ 8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_eq!(u.determinant(), 1032.0);
    }
}