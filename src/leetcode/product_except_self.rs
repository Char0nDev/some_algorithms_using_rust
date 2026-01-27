pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![1; n];

    let mut left_prod = 1;
    for i in 0..n {
        res[i] = left_prod;
        left_prod *= nums[i];
    }

    let mut right_prod = 1;
    for i in (0..n).rev() {
        res[i] *= right_prod;
        right_prod *= nums[i];
    }

    res
}
