#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [f32; N],
        B: [f32; N]
    }

    let result = A.iter().zip(B).map(|(a, b)| (a + b * 2.) / 3.).sum::<f32>();
    println!("{:?}", result);
}
