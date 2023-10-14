#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn isprime(n: u32) -> bool {
    for i in 2..=n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    input! {
        N: u32,
    }
    let mut answer: Vec<u32> = vec![];
    for n in 2..=N {
        if isprime(n) {
            answer.push(n)
        };
    }
    println!("{}", answer.iter().join(" "));
}
