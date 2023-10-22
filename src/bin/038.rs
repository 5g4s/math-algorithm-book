#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A:[u64; N],
        LR: [(usize, usize); Q]
    }

    let mut accumulation = vec![0; N + 1];
    for i in 0..N {
        accumulation[i + 1] = A[i] + accumulation[i]
    }

    for (L, R) in LR {
        println!("{}", accumulation[R] - accumulation[L - 1]);
    }
}
