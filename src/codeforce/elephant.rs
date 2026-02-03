/* https://codeforces.com/problemset/problem/617/A */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut x: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let possible_steps = [1, 2, 3, 4, 5];
    let mut current_index = 4;
    let mut steps = 0;

    while x != 0 {
        let rest = x % possible_steps[current_index];

        if rest != 0 {
            steps += x / possible_steps[current_index] as usize;
            x = x - (x / possible_steps[current_index] as usize) * possible_steps[current_index];

            current_index -= 1;
            continue;
        }

        if rest == 0 {
            steps += x / possible_steps[current_index] as usize;
            break;
        }
    }
    println!("{}", steps);
}
