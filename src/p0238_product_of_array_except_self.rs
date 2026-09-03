// https://leetcode.com/problems/product-of-array-except-self/description/

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];
    let mut prefix = 1;
    let mut suffix = 1;

    for i in 0..n {
        result[i] = prefix;
        prefix *= nums[i];
        // println!("prefix pass  i={i}  wrote result[{i}], prefix now = {prefix}");
    }
    // println!("after prefix loop: prefix = {prefix}, result = {result:?}");
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= nums[i];
        // println!("suffix pass  i={i}  result[{i}]={}, suffix now = {suffix}", result[i]);
    }
    // println!("after suffix loop: suffix = {suffix}, result = {result:?}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    // #[test]
    // fn example_2() {
    //     assert_eq!(product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    // }
}
