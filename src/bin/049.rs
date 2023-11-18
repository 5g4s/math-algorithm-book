#![allow(non_snake_case)]
const MOD: usize = 1_000_000_007;

use proconio::input;

fn main() {
    input! {
        N: usize
    }

    let mut a = [1, 1];

    for _ in 2..N {
        let mut ans = a[1] + a[0];
        if ans >= MOD {
            ans -= MOD;
        };
        a[0] = a[1];
        a[1] = ans;
    }
    println!("{}", a[1]);
}
