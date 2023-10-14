#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u16,
        A: [u16; N]
    }
    println!("{}", A.iter().sum::<u16>());
}
