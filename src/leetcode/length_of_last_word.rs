pub fn length_of_last_word(s: String) -> i32 {
    let new_str: Vec<&str> = s.trim().split(" ").collect();
    let lenght_of_the_last = (new_str[new_str.len() - 1].len()) as i32;

    return lenght_of_the_last;
}
