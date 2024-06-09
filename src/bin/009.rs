#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }

    let mut dp = vec![false; S+1];
    dp[0] = true;

    for &a in &A{
        for i in (0..=S).rev(){
            if dp[i] && i + a <= S {
                dp[i + a] = true;
            }
        }
    }

    if dp[S] {
        println!("Yes");
    } else {
        println!("No");
    }
}
