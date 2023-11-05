#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        R: usize,
        C: usize,
        (sy, sx): (usize, usize),
        (gy, gx): (usize, usize),
        m: [Chars; R]
    }

    let mut dist = vec![vec![-1; C]; R];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_back((sy - 1, sx - 1));
    dist[sy - 1][sx - 1] = 0;

    while let Some((now_y, now_x)) = que.pop_front() {
        let directions = [
            (now_y - 1, now_x),
            (now_y + 1, now_x),
            (now_y, now_x - 1),
            (now_y, now_x + 1),
        ];
        for (dy, dx) in directions {
            if m[dy][dx] == '.' && dist[dy][dx] == -1 {
                dist[dy][dx] = dist[now_y][now_x] + 1;
                que.push_back((dy, dx));
            }
        }
    }
    println!("{}", dist[gy - 1][gx - 1]);
}
