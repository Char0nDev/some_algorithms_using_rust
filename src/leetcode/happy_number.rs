use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut sum = 0;
    let mut n_to_vec: Vec<i32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut hash_set_nbr: HashSet<i32> = HashSet::new();

    while sum != 1 {
        for elm in &n_to_vec {
            sum += elm * elm;
        }

        if hash_set_nbr.contains(&sum) {
            return false;
        }

        hash_set_nbr.insert(sum);
        n_to_vec = sum
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        if sum == 1 {
            return true;
        }

        sum = 0;
    }

    return false;
}
