#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: [i32; 3]
    }
    println!("{}", A.iter().product::<i32>());
}
