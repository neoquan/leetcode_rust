// https://leetcode.com/problems/continuous-subarray-sum/

use std::collections::HashMap;

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut first_index: HashMap<i32, i32> = HashMap::new();

    first_index.insert(0, -1);

    let mut running_sum = 0;

    for (i, num) in nums.iter().enumerate() {
        running_sum += num;

        let remainder = ((running_sum % k) + k) % k;

        if let Some(&prev_index) = first_index.get(&remainder) {
            if i as i32 - prev_index >= 2 {
                return true;
            }
        } else {
            first_index.insert(remainder, i as i32);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    }
}
