use crate::core::matrix::Matrix;
use crate::core::vector::Vector;

mod core;

fn main() {
    let ma = Matrix::from_rows([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0]
    ]);

    println!("{}", ma);
    println!("{}", ma);

    let a = Vector::from([1.0, 2.0, 3.0]);
    let b = Vector::from([4.0, 5.0, 6.0]);

    println!("{}", a);
    println!("{}", b);
}
