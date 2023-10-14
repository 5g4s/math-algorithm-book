#![allow(non_snake_case)]

use proconio::input;


fn main() {
    input! {
        N: i64,
        A: [usize; N]
    }

    let mut result: Vec<i64> = vec![0; 4];
    for i in A.iter() {
        result[*i] += 1;
    }
    let mut answer = 0;
    for n in result.iter() {
        answer += n * (n - 1) / 2;
    }
    println!("{}", answer);
}
