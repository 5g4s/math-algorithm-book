#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        PQ: [(i32, i32); N],
    }

    let result = PQ
        .iter()
        .map(|&(p, q)| (q as f32) / (p as f32))
        .sum::<f32>();
    println!("{:.?}", result);
}
