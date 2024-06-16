#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [i32; N],
        B: [i32; N]
    }

    let result = A
        .iter()
        .zip(B.iter())
        .map(|(&a, &b)| (a as f32 + b as f32 * 2.) / 3.)
        .sum::<f32>();
    println!("{:?}", result);
}
