pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let map = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let mut result = vec!["".to_string()];

    for digit in digits.chars() {
        let digit_idx = digit.to_digit(10).unwrap() as usize;
        let letter = map[digit_idx];
        let mut new_combinitions = Vec::new();

        for combination in result {
            for letter in letter.chars() {
                new_combinitions.push(format!("{}{}", combination, letter));
            }
        }
        result = new_combinitions;
    }

    return result;
}
