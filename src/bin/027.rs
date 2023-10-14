#![allow(non_snake_case)]

use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        N: usize,
        mut A: [u64; N],
    }

    A.sort_unstable();
    println!("{}", A.iter().join(" "));
}
