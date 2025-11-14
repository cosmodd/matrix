use crate::linalg::linear_combination;
use crate::vector::Vector;

mod vector;
mod matrix;
mod linalg;

fn main() {
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    let lc = linear_combination(&[v1, v2], &[10.]).unwrap();

    println!("Linear combination: {}", lc);
}
