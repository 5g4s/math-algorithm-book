#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        PQ: [(f32, f32); N],
    }

    let result = PQ.iter().map(|(p,q)| q / p).sum::<f32>();
    println!("{:?}", result);
}
