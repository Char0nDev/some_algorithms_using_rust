use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let str_vec: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (n, m) = (str_vec[0], str_vec[1] as usize);

    for i in 0..n {
        if i % 2 == 0 {
            println!("{}", "#".repeat(m));
        } else {
            if (i / 2) % 2 == 0 {
                println!("{}#", ".".repeat(m - 1));
            } else {
                println!("#{}", ".".repeat(m - 1));
            }
        }
    }
}
