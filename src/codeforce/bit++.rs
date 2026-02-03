use std::io::{self, BufRead};

/* https://codeforces.com/contest/282/problem/A */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut x = 0;
    for _ in 1..=n {
        let str = lines.next().and_then(|l| l.ok()).unwrap();
        let remove_x = str.replace("X", "");

        if &remove_x == "--" {
            x -= 1;
        } else if &remove_x == "++" {
            x += 1;
        }
    }
    println!("{}", x);
}
