use std::io;

fn is_distinct(mut n: i32) -> bool {
    let mut used = [false; 10];
    while n > 0 {
        let digit = (n % 10) as usize;
        if used[digit] {
            return false;
        }
        used[digit] = true;
        n /= 10;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut year: i32 = input.trim().parse().unwrap();

    loop {
        year += 1;
        if is_distinct(year) {
            println!("{}", year);
            break;
        }
    }
}
