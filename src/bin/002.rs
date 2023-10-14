#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        A: [u16; 3]
    }
    println!("{}", A.iter().sum::<u16>());
}
