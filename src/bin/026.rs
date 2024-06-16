#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u64,
    }

    let mut result = 0.0;
    for i in 1..=N {
        result += N as f64 / i as f64;
    }
    println!("{:?}", result);
}
