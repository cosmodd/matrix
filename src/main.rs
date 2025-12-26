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
    let mc = ma.clone() + mb.clone();
    let md = mb.clone() - ma.clone();
    let me = ma.clone() * 5.;

    println!("{}", ma);
    println!("{}", mb);
    println!("{}", mc);
    println!("{}", md);
    println!("{}", me);
    println!("ma == ma = {}", ma == ma);
    println!("ma == mb = {}", ma == mb);
    println!("ma != ma = {}", ma != ma);
    println!("ma != mb = {}", ma != mb);

    let a = Vector::from([1.0, 2.0, 3.0]);
    let b = Vector::from([4.0, 5.0, 6.0]);
    let c = Vector::from([7.0, 8.0, 9.0, 10.0]);
    let d = Vector::from([11.0, 12.0, 13.0, 14.0]);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", a.clone() + b.clone());
    println!("{}", c.clone() + d.clone());
    println!("{}", a.clone() - b.clone());
    println!("{}", c.clone() - d.clone());
    println!("{}", a.clone() * 10.);
    println!("{}", c.clone() * 10.);
}
