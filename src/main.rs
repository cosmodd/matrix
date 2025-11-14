use crate::matrix::Matrix;

mod vector;
mod matrix;

fn main() {
    let mut mat = Matrix::from([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ]);

    println!("{:?}", mat);
    println!("{}", mat);
}
