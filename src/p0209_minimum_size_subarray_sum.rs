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
            left += 1
        }
    }
    if best == usize::MAX { 0 } else { best as i32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}
