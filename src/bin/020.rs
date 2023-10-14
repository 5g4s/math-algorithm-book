#![allow(non_snake_case)]

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        mut A: [i32; N]
    }

    A.sort();
    let mut answer = 0;
    for i in 0..N{
        for j in i+1..N{
            for k in j+1..N{
                for l in k+1..N{
                    let n = 1000 - A[i] - A[j] - A[k] - A[l];
                    answer += A[l+1..].equal_range(&n).len();
                }
            }
        }
    }
    println!("{}", answer);
}
