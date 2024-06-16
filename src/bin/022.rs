#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    let mut L = [0; 100000];
    for &i in &A {
        L[i] += 1;
    }

    let mut result: u64 = 0;
    for j in 1..50000 {
        result += L[j] * L[100000 - j];
    }
    result += L[50000] * (L[50000] - 1) / 2;

    println!("{}", result);
}
