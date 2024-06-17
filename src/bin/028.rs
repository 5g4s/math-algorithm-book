#![allow(non_snake_case)]

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        N: usize,
        h: [i32; N],
    }

    let mut dp = vec![0; N];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..N {
        dp[i] = min(
            dp[i - 1] + (h[i] - h[i - 1]).abs(),
            dp[i - 2] + (h[i] - h[i - 2]).abs(),
        );
    }
    println!("{}", dp[N - 1]);
}
