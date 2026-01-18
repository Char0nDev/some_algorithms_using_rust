pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let nums_len = nums.len() - 1;
    if nums.is_empty() {
        return 0;
    }
    for i in 0..=nums_len {
        if nums[i] == val {
            let temp = nums[i].clone();
            nums[i] = nums[nums_len];
            nums[nums_len] = temp;
        }
    }

    *nums = nums.iter().filter(|c| **c != val).copied().collect();

    return nums.len() as i32;
}
