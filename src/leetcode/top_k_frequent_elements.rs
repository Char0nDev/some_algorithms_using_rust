use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    let mut output: Vec<i32> = Vec::new();

    for elm in nums {
        if let Some(i) = hash_map.get_mut(&elm) {
            *i += 1;
        } else {
            hash_map.insert(elm, 1);
        }
    }

    let mut hash_map_vec: Vec<(i32, i32)> = hash_map.iter().map(|(&k, &v)| (k, v)).collect();
    hash_map_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for i in 0..k {
        output.push(hash_map_vec[i as usize].0);
    }

    return output;
}
