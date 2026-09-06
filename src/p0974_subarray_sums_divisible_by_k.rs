// https://leetcode.com/problems/subarray-sums-divisible-by-k/description/

use std::collections::HashMap;

pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut remainder_counts: HashMap<i32, i32> = HashMap::new();

    remainder_counts.insert(0, 1);

    let mut running_sum = 0;
    let mut result = 0;

    for num in nums {
        running_sum += num;

        let remainder = ((running_sum % k) + k) % k;

        println!("For num = {num}, the remainder is {remainder}");

        if let Some(&count) = remainder_counts.get(&remainder) {
            result += count;
        }

        *remainder_counts.entry(remainder).or_insert(0) += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }

    #[test]
    fn example_2() {
        assert_eq!(subarrays_div_by_k(vec![5], 9), 0);
    }
}
