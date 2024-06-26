#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut B: [f64; N],
        mut R:[f64; N]
    }

    let result = (B.iter().sum::<f64>() + R.iter().sum::<f64>()) / N as f64;
    println!("{}", result);
}
