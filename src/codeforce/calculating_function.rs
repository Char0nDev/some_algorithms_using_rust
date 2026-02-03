use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().unwrap().parse::<isize>().unwrap();

    if n % 2 == 0 {
        println!("{}", n / 2);
    } else {
        println!("{}", -(n + 1) / 2);
    }
}

// when n is odd so S = (-1 + 2) + (-3 + 4) + ..... + (-(n-1) + n) the different is 1 in every () and we have n number so the sum is n/2
// when n is odd so S = (-1 + 2) + (-3 + 4) + ..... + (-(n-2) + n-1) - n the different is 1 in every () and we have n number so the sum is (n-1)/2 - n
