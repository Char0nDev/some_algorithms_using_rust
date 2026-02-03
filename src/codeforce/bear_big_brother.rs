use std::io::{self, BufRead};

/* https://codeforces.com/problemset/problem/791/A */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    let (mut limak_w, mut bob_w) = (first_line[0], first_line[1]);
    let mut year = 0;

    while limak_w <= bob_w {
        limak_w = limak_w * 3;
        bob_w = bob_w * 2;
        year += 1;
    }

    println!("{}", year);
    // by default limak_w <= bob_w
    // every year limak_w triple sa w
    // and bob double sa weight
}
