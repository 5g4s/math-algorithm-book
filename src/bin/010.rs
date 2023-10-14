#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u64,
    }

    let mut answer = 1;
    for n in 1..=N {
        answer *= n;
    }
    println!("{}", answer);
}
