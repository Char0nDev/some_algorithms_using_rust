use std::{
    cmp,
    io::{self, BufRead},
};

/* https://codeforces.com/contest/2179/problem/B */

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<u32>().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let arr: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut total_dist: i64 = 0;

        // here we calculet total distance
        for i in 0..n - 1 {
            total_dist += (arr[i] - arr[i + 1]).abs() as i64;
        }

        // calcul the best saving when we skip the first one and the the last one
        let mut max_savings = cmp::max((arr[0] - arr[1]).abs(), (arr[n - 1] - arr[n - 2]).abs());

        for i in 1..n - 1 {
            let current_path = (arr[i - 1] - arr[i]).abs() + (arr[i] - arr[i + 1]).abs();
            let shortcut = (arr[i - 1] - arr[i + 1]).abs();
            let savings = current_path - shortcut;

            if savings > max_savings {
                max_savings = savings;
            }
        }

        println!("{}", total_dist - max_savings as i64);
    }
}
