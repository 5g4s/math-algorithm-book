#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u64,
    }

    for n in 1.. {
        if n * n > N {
            break;
        }
        if N % n == 0 {
            println!("{}", n);
            if (N / n) != n {
                println!("{}", N / n);
            }
        }
    }
}
