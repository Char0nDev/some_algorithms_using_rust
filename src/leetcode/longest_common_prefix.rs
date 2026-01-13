fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone();

    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    prefix
}

// another mehode
/*
if strs.is_empty() {
    return String::new();
}

let first = strs[0].clone();

for (i, c) in first.chars().enumerate() {
    for s in &strs[1..] {
        if s.chars().nth(i) != Some(c) {
            return first[..i].to_string();
        }
    }
}

first
*/
