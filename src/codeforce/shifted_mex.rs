use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: u32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let _: u32 = lines.next().unwrap().unwrap().parse().unwrap();
        let mut arr: Vec<i64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        arr.sort();
        arr.dedup();

        let mut current_streak = 1;
        let mut max_streak = 0;

        for i in 0..arr.len() {
            if i > 0 && arr[i] == arr[i - 1] + 1 {
                current_streak += 1;
            } else {
                current_streak = 1
            }

            if current_streak > max_streak {
                max_streak = current_streak;
            }
        }

        println!("{}", max_streak);
    }
}
