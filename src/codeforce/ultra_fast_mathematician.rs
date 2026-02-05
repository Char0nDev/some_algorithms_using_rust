use std::io::{self, BufRead};

/* https://codeforces.com/problemset/problem/61/Az   */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1 = lines.next().unwrap().unwrap();
    let line1 = line1.trim();

    let line2 = lines.next().unwrap().unwrap();
    let line2 = line2.trim();

    let result: String = line1
        .to_string()
        .chars()
        .zip(line2.to_string().chars())
        .map(|(c1, c2)| if c1 != c2 { '1' } else { '0' })
        .collect();

    println!("{}", result);
}
