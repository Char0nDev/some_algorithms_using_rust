use std::io::{self, BufRead};

/* https://codeforces.com/problemset/problem/996/A */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let denominations = [100, 20, 10, 5, 1];
    let mut total_bills = 0;

    for &bill in denominations.iter() {
        total_bills += n / bill;
        n %= bill;
    }

    println!("{}", total_bills);
}
