use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
    let t: usize = first_line.trim().parse().unwrap_or(0);

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let arr_str = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let mut arr: Vec<usize> = arr_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let max_index = arr
            .iter()
            .enumerate()
            .max_by_key(|&(_index, value)| value)
            .map(|(index, _value)| index)
            .unwrap();

        arr.swap(0, max_index);

        let mut max = 0;
        for i in 0..arr.len() {
            let get_max_of_arr = &arr[0..=i].iter().max();
            max += get_max_of_arr.unwrap();
        }
        println!("{}", max);
    }
}
