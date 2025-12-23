use crate::core::matrix::Matrix;

mod core;

fn main() {
    let a = Matrix::from([
        [1.1, 2.2, 3.3],
        [4.4, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ]);

    println!("{:?}", a);
    println!("{}", a);
}
