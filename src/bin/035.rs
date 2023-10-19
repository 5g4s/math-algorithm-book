#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        xyr: [(f64, f64, f64); 2]
    }

    let d2 = ((xyr[0].0 - xyr[1].0).powf(2.0) + (xyr[0].1 - xyr[1].1).powf(2.0)).sqrt();

    let l = xyr[0].2 + xyr[1].2;
    let l_abs = (xyr[0].2 - xyr[1].2).abs();

    let answer = if d2 < l_abs {
        1
    } else if d2 == l_abs {
        2
    } else if d2 < l {
        3
    } else if d2 == l {
        4
    } else {
        5
    };
    println!("{}", answer);
}
