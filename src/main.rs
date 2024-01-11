extern crate nalgebra as na;
extern crate time;
use na::{Vector3};
use std::env;
use time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for correct number of arguments
    if args.len() != 3 {
        println!("Usage: program <mode: trig/no-trig> <iterations>");
        return;
    }

    let mode = &args[1];
    let iterations: usize = args[2].parse().expect("Please provide a valid number for iterations");

    match mode.as_str() {
        "trig" => run_calculations("trig", iterations),
        "no-trig" => run_calculations("no-trig", iterations),
        _ => println!("Invalid mode. Please use 'trig' or 'no-trig'."),
    }
}

fn run_calculations(mode: &str, iterations: usize) {
    let mut total_time = 0;

    for i in 0..iterations {
        let v1 = Vector3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());
        let v2 = Vector3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());

        if i % (iterations / 10) == 0 {
            println!("{}%", i / (iterations / 100));
        }

        let start = Instant::now();
        match mode {
            "trig" => { let _ = calculate_with_trig(v1, v2); },
            "no-trig" => { let _ = calculate_without_trig(v1, v2); },
            _ => unreachable!(), // This case should never happen
        }
        total_time += start.elapsed().whole_nanoseconds();
    }

    println!("Total time for {} mode: {} nanoseconds", mode, total_time);
}

fn calculate_with_trig(v1: Vector3<f64>, v2: Vector3<f64>) -> f64 {
    let alpha = (v1.dot(&v2) / (v1.norm() * v2.norm())).acos();
    let area = 0.5 * v1.norm() * v2.norm() * alpha.sin();
    return area;
}

fn calculate_without_trig(v1: Vector3<f64>, v2: Vector3<f64>) -> f64 {
    let area = 0.5 * (v1.norm().powi(2) * v2.norm().powi(2) - v1.dot(&v2).powi(2)).sqrt();
    return area;
}

// fn calculate_with_cross(v1: Vector3<f64>, v2: Vector3<f64>, v3: Vector3<f64>) -> f64 {
//     let area = 0.5 * v1.cross(&v2).norm();
//     return area;
// }
