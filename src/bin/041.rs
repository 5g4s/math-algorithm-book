#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        T: usize,
        N: usize,
        LR: [(usize, usize); N]
    }

    let mut accumulation = vec![0; T + 1];

    for (L, R) in LR {
        accumulation[L] += 1;
        accumulation[R] += -1;
    }

    let mut ans = 0;
    for j in &accumulation[0..T] {
        ans += j;
        println!("{}", ans);
    }
}
