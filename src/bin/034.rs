#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        xy: [(f64, f64); N]
    }

    let mut answer = std::f64::MAX;
    for i in 0..N {
        for j in i + 1..N {
            let d = (xy[i].0 - xy[j].0).powf(2.0) + (xy[i].1 - xy[j].1).powf(2.0);
            if answer > d {
                answer = d
            }
        }
    }

    println!("{}", answer.sqrt());
}
