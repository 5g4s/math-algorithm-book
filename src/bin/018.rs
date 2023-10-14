#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u64,
        A: [usize; N]
    }

    let mut result: Vec<u64> = vec![0; 5];
    for i in A.iter() {
        result[i / 100] += 1;
    }
    println!("{}", result[1] * result[4] + result[2] * result[3]);
}
