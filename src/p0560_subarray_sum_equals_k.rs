// https://leetcode.com/problems/subarray-sum-equals-k/description/

use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    // prefix_sum_counts = every running total I've seen so far,
    // and how many times I've seen it.
    let mut prefix_sum_counts: HashMap<i32, i32> = HashMap::new();

    prefix_sum_counts.insert(0, 1);
    let mut running_sum = 0;
    let mut result = 0;

    for num in nums {
        running_sum += num;

        let target = running_sum - k;

        if let Some(count) = prefix_sum_counts.get(&target) {
            result += count;
        }

        *prefix_sum_counts.entry(running_sum).or_insert(0) += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
