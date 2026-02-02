use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let arr_str = lines.next().and_then(|l| l.ok()).unwrap();
        let str_len = arr_str.trim().len() as i32;

        if str_len <= 10 {
            println!("{}", arr_str);
            continue;
        }

        let arr_str: Vec<char> = arr_str.chars().collect();

        println!(
            "{}{}{}",
            arr_str[0],
            arr_str[1..=(arr_str.len() - 2)].len(),
            arr_str[arr_str.len() - 1]
        );
    }
}
