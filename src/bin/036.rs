#![allow(non_snake_case)]

use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        A: f64,
        B: f64,
        H: f64,
        M: f64
    }

    let short = ((H / 12.0) + (M / (60.0 * 12.0))) * 2.0 * PI;
    let long = (M / 60.0) * 2.0 * PI;
    let theta = short - long;
    let answer = A.powf(2.0) + B.powf(2.0) - 2.0 * A * B * theta.cos();
    println!("{}", answer.sqrt());
}
