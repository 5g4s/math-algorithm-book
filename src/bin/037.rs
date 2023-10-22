#![allow(non_snake_case)]

use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
    xy1: (i64, i64),
    xy2: (i64, i64),
    xy3: (i64, i64),
    xy4: (i64, i64),
    }

    let a = (xy2.0 - xy1.0) * (xy3.1 - xy1.1) - (xy2.1 - xy1.1) * (xy3.0 - xy1.0);
    let b = (xy2.0 - xy1.0) * (xy4.1 - xy1.1) - (xy2.1 - xy1.1) * (xy4.0 - xy1.0);
    let c = (xy4.0 - xy3.0) * (xy1.1 - xy3.1) - (xy4.1 - xy3.1) * (xy1.0 - xy3.0);
    let d = (xy4.0 - xy3.0) * (xy2.1 - xy3.1) - (xy4.1 - xy3.1) * (xy2.0 - xy3.0);

    if a == 0 && b == 0 && c == 0 && d == 0 {
        let max_12 = max(xy1, xy2);
        let min_12 = min(xy1, xy2);
        let max_34 = max(xy3, xy4);
        let min_34 = min(xy3, xy4);

        let ans = if max(min_12, min_34) <= min(max_12, max_34) {
            "Yes"
        } else {
            "No"
        };
        println!("{}", ans);
        return;
    }

    if a.signum() * b.signum() <= 0 && c.signum() * d.signum() <= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
