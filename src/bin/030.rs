#![allow(non_snake_case)]

use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        N: usize,
        W: usize,
        wv: [(usize, usize); N]
    }

    let mut dp = vec![0; W + 1];
    for (w, v) in wv {
        for j in (w..=W).rev() {
            dp[j] = max(dp[j], dp[j - w] + v);
        }
    }

    println!("{}", dp[W]);
}
