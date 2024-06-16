#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
        S: u32,
    }

    let mut answer: u32 = 0;
    for red in 1..=N {
        for blue in 1..=N {
            if red + blue <= S {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
