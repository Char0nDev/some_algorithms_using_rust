use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut output = HashMap::new();

    for str in strs {
        let mut chars: Vec<char> = str.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        output.entry(key).or_insert(Vec::new()).push(str);
    }
    return output.into_values().collect();
}
