#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [u32; N],
    }

    A.sort_unstable();
    println!("{}", A.iter().join(" "));
}
