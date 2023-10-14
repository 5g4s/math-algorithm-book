#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        n: u64,
        r: u64
    }

    let mut denominator = 1;
    let mut numerator = 1;
    
    for i in 2..=r{
        denominator *= i;
    }

    for j in n-r+1..=n{
        numerator *= j
    }

    let result = numerator / denominator;
    println!("{}", result);
}
