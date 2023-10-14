#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut N: u64,
    }

    let mut answer: Vec<u64> = vec![];
    let mut i = 2;
    while i * i <= N {
        if N % i == 0 {
            answer.push(i);
            N /= i;
        } else {
            i += 1
        }
    }
    if N > 2{
        answer.push(N);
    }
    println!("{}", answer.iter().join(" "));
}
