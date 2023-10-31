#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut ans = 0;
    for i in 1..=N {
        ans += i * (N / i) * (1 + (N / i)) / 2;
    }
    println!("{}", ans);
}
