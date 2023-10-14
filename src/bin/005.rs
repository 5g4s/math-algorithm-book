#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: u32,
        a: [u32; N]
    }
    println!("{}", a.iter().sum::<u32>() % 100);
}
