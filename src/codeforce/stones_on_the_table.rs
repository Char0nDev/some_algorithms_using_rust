use std::io::{self, BufRead};

/* https://codeforces.com/problemset/problem/266/A */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n_input = lines.next().unwrap().unwrap();
    let _n: usize = n_input.trim().parse().unwrap();

    let s = lines.next().unwrap().unwrap();
    let s = s.trim();

    let mut removals = 0;
    let bytes = s.as_bytes();

    for i in 0..bytes.len().saturating_sub(1) {
        if bytes[i] == bytes[i + 1] {
            removals += 1;
        }
    }

    println!("{}", removals);
}
