use crate::core::ops::perspective_projection;

mod core;
mod traits;

fn main() {
    let perspective = perspective_projection(45.0, 1.0, 1.0, 1000.0);
    let (w, h) = perspective.shape();

    for y in 0..h {
        for x in 0..w {
            let value: f64 = perspective[(x, y)];
            if value.fract() == 0.0 {
                print!("{}.", value);
            } else {
                print!("{}", value);
            }

            if x < w - 1 {
                print!(", ");
            }
        }
        println!();
    }
}
