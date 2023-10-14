#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        mut A: u64,
        mut B: u64,
    }


    while (A > 0) && (B > 0) {
        if A > B{
            A %= B;
        } else{
            B %= A;
        }
    }

    if B == 0{
        println!("{}", A);
    } else {
        println!("{}", B);
    }
}