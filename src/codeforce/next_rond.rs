/* https://codeforces.com/problemset/problem/158/A */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (_, k) = (first_line[0], first_line[1]);
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let k_index = (k - 1) as usize;
    let score_to_beat = arr[k_index];

    let mut count = 0;

    for &score in arr.iter() {
        if score > 0 && score >= score_to_beat {
            count += 1;
        }
    }

    println!("{}", count);
}
