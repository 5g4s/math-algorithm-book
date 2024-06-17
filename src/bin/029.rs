#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let mut dp = vec![0; N + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=N {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("{}", dp[N]);
}
