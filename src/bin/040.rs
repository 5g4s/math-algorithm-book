#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [i64; N-1],
        M: usize,
        B: [usize; M]
    }

    let mut accumulation = vec![0; N + 1];

    for i in 0..N - 1 {
        accumulation[i + 1] = accumulation[i] + A[i];
    }

    let mut ans = 0;
    for j in 0..M-1 {
        ans += (accumulation[B[j + 1] - 1] - accumulation[B[j] - 1]).abs();
    }
    println!("{}", ans);
}
