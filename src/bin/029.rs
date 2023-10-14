#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let mut dp = vec![1; N + 1];
    for i in 2..=N {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("{}", dp[N]);
}
