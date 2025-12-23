#![allow(dead_code)]

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Matrix<K> {
    shape: (usize, usize),
    data: Vec<K>,
}

impl<K> Matrix<K>
where
    K: Clone,
{
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
                data.push(values[y][x].clone());
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
                data.push(values[x][y].clone());
            }
        }

        Matrix {
            shape: (W, H),
            data,
        }
    }
}

impl<K> Clone for Matrix<K>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Matrix {
            shape: self.shape,
            data: self.data.clone(),
        }
    }
}

impl<K> fmt::Display for Matrix<K>
where
    K: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
