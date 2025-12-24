use crate::core::matrix::Matrix;
use crate::core::vector::Vector;

mod core;
mod traits;

fn main() {
    let ma = Matrix::from_rows([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0]
    ]);
    let mb = Matrix::from_rows([
        [1.0, 2.0, 3.0],
        [4.0, 0.0, 6.0],
        [7.0, 8.0, 9.0]
    ]);

    println!("{}", ma);
    println!("{}", mb);
    println!("ma == ma = {}", ma == ma);
    println!("ma == mb = {}", ma == mb);
    println!("ma != ma = {}", ma != ma);
    println!("ma != mb = {}", ma != mb);

    let a = Vector::from([1.0, 2.0, 3.0]);
    let b = Vector::from([4.0, 5.0, 6.0]);

    println!("{}", a);
    println!("{}", b);
}
