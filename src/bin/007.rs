#![allow(non_snake_case)]

use proconio::input;
use num::integer::lcm;

fn main() {
    input!{
        N: i32,
        X: i32,
        Y: i32
    }

    let answer = (N / X) + (N / Y) - (N / lcm(X, Y)); 
    println!("{}", answer);
}