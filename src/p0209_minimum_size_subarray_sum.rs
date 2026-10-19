// https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=problem-list-v2&envId=prefix-sum


pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut best = usize::MAX;
    let mut left = 0;
    let mut sum = 0;

    for right in 0..nums.len() {
        sum += nums[right];
        while sum >= target {
            best = best.min(right - left + 1);
            sum -= nums[left];
            left += 1;
        }
    }
    if best == usize::MAX {0} else {best as i32 }
}