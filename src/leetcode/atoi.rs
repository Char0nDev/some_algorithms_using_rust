pub fn my_atoi(s: String) -> i32 {
    let mut is_negative = false;
    let mut nbr: Vec<char> = Vec::new();
    let mut has_sign = false;

    for char in s.trim().chars() {
        if char == '_' {
            continue;
        } else if char.is_digit(10) {
            nbr.push(char);
        } else if (char == '-' || char == '+') && nbr.len() == 0 {
            if has_sign {
                break;
            }
            if char == '-' {
                is_negative = true;
            }
            has_sign = true;
            continue;
        } else if (char == '-' || char == '+') && nbr.len() != 0 {
            break;
        } else if char.is_alphabetic() {
            break;
        } else {
            break;
        }
    }

    if nbr.is_empty() {
        return 0;
    }

    let s_num: String = nbr.iter().collect();
    let parsed_large: i64 = s_num.parse().unwrap_or(i64::MAX);

    if is_negative {
        let final_val = -parsed_large;
        if final_val < i32::MIN as i64 {
            return i32::MIN;
        }
        return final_val as i32;
    } else {
        if parsed_large > i32::MAX as i64 {
            return i32::MAX;
        }
        return parsed_large as i32;
    }
}
