#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        X:u64,
        A: [u64; N]
    }

    if A.contains(&X){
        println!("Yes");
    } else{
        println!("No");
    }
}
