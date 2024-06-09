#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    let mut dp = vec![vec![0; 1001]; 6];
    dp[0][0] = 1;

    for &a in &A {
        for i in (1..=5).rev() {
            for j in (a..=1000).rev() {
                dp[i][j] += dp[i - 1][j - a];
            }
        }
    }
    println!("{}", dp[5][1000]);
}



