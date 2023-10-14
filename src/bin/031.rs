#![allow(non_snake_case)]

use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }

    let mut dp = vec![0; N];

    dp[0] = A[0];
    dp[1] = A[1];
    for i in 2..N {
        dp[i] = max(dp[i - 1], dp[i - 2] + A[i]);
    }
    println!("{}", dp[N - 1]);
}
