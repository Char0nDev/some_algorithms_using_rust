use std::io::{self, BufRead};

/* https://codeforces.com/problemset/problem/1367/B */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: u32 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();

    for _ in 0..t {
        let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let line = lines.next().unwrap().unwrap();
        let a: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut wrong_even = 0;
        let mut wrong_odd = 0;

        for (i, &val) in a.iter().enumerate() {
            if (i % 2) as i32 != val % 2 {
                if i % 2 == 0 {
                    wrong_even += 1;
                } else {
                    wrong_odd += 1;
                }
            }
        }

        if wrong_even == wrong_odd {
            println!("{}", wrong_even);
        } else {
            println!("-1");
        }
    }
}
