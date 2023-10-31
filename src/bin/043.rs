#![allow(non_snake_case)]

use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut ds = Dsu::new(N + 1);

    for (A, B) in AB {
        ds.merge(A, B);
    }

    if ds.size(1) == N {
        print!("The graph is connected.");
    } else {
        print!("The graph is not connected.");
    }
}
