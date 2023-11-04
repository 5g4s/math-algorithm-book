#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        ab: [(usize, usize); M]
    }

    let mut count = vec![0; N];
    for (a, b) in ab {
        if a < b {
            count[b-1] += 1;
        } else {
            count[a-1] += 1;
        }
    }

    let mut answer = 0;
    for i in count {
        if i == 1 {
            answer += 1;
        }
    }
    println!("{}", answer);
}
