use crate::linalg::{lerp_matrix, lerp_vector};
use crate::matrix::Matrix;
use crate::vector::Vector;

mod vector;
mod matrix;
mod linalg;

fn main() {
    println!("vec_lerp = {}", lerp_vector(&Vector::from([2., 1.]), &Vector::from([4., 2.]), 0.3).unwrap());
    println!("mat_lerp =\n{}", lerp_matrix(&Matrix::from([[2., 1.], [3., 4.]]), &Matrix::from([[20., 10.], [30., 40.]]), 0.5).unwrap());
}
