#![allow(non_snake_case)]

use proconio::{fastout, input};

fn dfs(color: &mut Vec<usize>, G: &Vec<Vec<usize>>, position: usize) {
    for &j in &G[position] {
        if color[j] == 0 {
            color[j] = 3 - color[position];
            dfs(color, G, j)
        }
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![0; 0]; N];
    for &(A, B) in &AB {
        G[A - 1].push(B - 1);
        G[B - 1].push(A - 1);
    }

    let mut color = vec![0; N];

    for i in 0..N {
        if color[i] == 0 {
            color[i] = 1;
            dfs(&mut color, &G, i)
        }
    }
    for &(A, B) in &AB {
        if color[A - 1] == color[B - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
