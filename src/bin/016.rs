#![allow(non_snake_case)]

use proconio::input;
use std::cmp::max;

fn GCD(mut A: u64, mut B: u64) -> u64 {
    while (A > 0) && (B > 0) {
        if A > B {
            A %= B;
        } else {
            B %= A;
        }
    }
    max(A, B)
}


fn main() {
    input! {
        N: u64,
        AN: [u64; N]
    }

    let mut result = AN[0];
    for i in 1..N {
        result = GCD(result, AN[i as usize]);
    }
    println!("{}", result);
}
