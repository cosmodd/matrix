#![allow(dead_code)]

use crate::traits::Field;
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

impl<K: Field> ops::Add for Matrix<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "Matrix addition dimensions mismatch.");

        let mut result = self.clone();

        for (a, b) in result.data.iter_mut().zip(rhs.data.iter()) {
            *a = *a + *b;
        }

        result
    }
}

impl<K: Field> ops::Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "Matrix substraction dimensions mismatch.");

        let mut result = self.clone();

        for (a, b) in result.data.iter_mut().zip(rhs.data.iter()) {
            *a = *a - *b;
        }

        result
    }
}

impl<K: Field> ops::Mul<K> for Matrix<K> {
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        let mut result = self.clone();

        for value in result.data.iter_mut() {
            *value = *value * rhs
        }

        result
    }
}