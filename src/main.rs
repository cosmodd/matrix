use crate::core::ops::perspective_projection;
use std::{env, process};

mod core;
mod traits;

fn parse_arg(args: &[String], index: usize, name: &str) -> f64 {
    args.get(index)
        .unwrap_or_else(|| {
            eprintln!("Missing argument '{name}'");
            eprintln!("Usage: cargo run -- <fov> <aspect> <near> <far>");
            process::exit(1);
        })
        .parse::<f64>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid '{name}': must be a number");
            eprintln!("Usage: cargo run -- <fov> <aspect> <near> <far>");
            process::exit(1);
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: cargo run -- <fov> <aspect> <near> <far>");
        eprintln!("Example: cargo run -- 45 1 1 100");
        process::exit(1);
    }
    let fov: f64 = parse_arg(&args, 1, "fov");
    let aspect: f64 = parse_arg(&args, 2, "aspect");
    let near: f64 = parse_arg(&args, 3, "near");
    let far: f64 = parse_arg(&args, 4, "far");

    let perspective = perspective_projection(fov, aspect, near, far);
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
