/*https://codeforces.com/contest/2180/problem/B */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let t: usize = line.trim().parse().unwrap();

        for _ in 0..t {
            let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

            if let Some(Ok(line)) = lines.next() {
                let a: Vec<&str> = line.split_whitespace().collect();
                let mut s = String::new();

                for &ai in &a {
                    if s.is_empty() {
                        s = ai.to_string();
                    } else {
                        let opt1 = format!("{}{}", ai, s);
                        let opt2 = format!("{}{}", s, ai);

                        if opt1 < opt2 { s = opt1 } else { s = opt2 }
                    }
                }

                println!("{}", s);
            }
        }
    };
}
