#![allow(non_snake_case)]

use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![0; 0]; N];
    for (A, B) in AB {
        G[A - 1].push(B - 1);
        G[B - 1].push(A - 1);
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![-1; N];
    dist[0] = 0;

    que.push_back(0);

    while let Some(now) = que.pop_front() {
        for &g in &G[now] {
            if dist[g] == -1 {
                dist[g] = dist[now] + 1;
                que.push_back(g);
            }
        }
    }
    for d in dist {
        println!("{}", d);
    }
}
