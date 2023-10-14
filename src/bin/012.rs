#![allow(non_snake_case)]

use proconio::input;

fn isprime(n: u64) -> bool {
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
        N: u64,
    }
    if isprime(N) {
        println!("Yes");
    }
    else {
        println!("No");

    }
}
