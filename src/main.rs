use crate::vector::Vector;

mod traits;
mod vector;
mod matrix;
mod common;

fn main() {
    let a = Vector::from([0.0, 10.0, 20.0, 30.0, 40.0]);
    let b = Vector::from([1.0, 2.0, 3.0, 4.0, 5.0]);
    let scalar = 5.;
    println!("   {a}");
    println!(" + {b}");
    println!(" = {}", a.clone() + b.clone());
    println!("");
    println!("   {a}");
    println!(" - {b}");
    println!(" = {}", a.clone() - b.clone());
    println!("");
    println!("   {a} * {scalar}");
    println!(" = {}", a.clone() * scalar);
}
