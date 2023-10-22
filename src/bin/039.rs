#![allow(non_snake_case)]

use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        LRX: [(usize, usize, i64); Q]
    }

    let mut diff = vec![0; N + 1];

    for (L, R, X) in LRX {
        diff[L - 1] += X;
        diff[R] -= X;
    }
    for i in &diff[1..N] {
        match i.cmp(&0) {
            Ordering::Less => print!(">"),
            Ordering::Equal => print!("="),
            Ordering::Greater => print!("<"),
        }
    }
}
