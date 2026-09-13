// https://leetcode.com/problems/set-mismatch/description/?envType=problem-list-v2&envId=dsa-linear-shoal-array-ii

// pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
//     let n = nums.len();
//     let mut seen: HashSet<i32> = HashSet::new();
//     let mut duplicate = 0;
//     let mut missing = 0;

//     for x in &nums {
//         if !seen.insert(*x) {
//             duplicate = *x;
//         }
//     }

//     for v in 1..=(n as i32) {
//         if !seen.contains(&v) {
//             missing = v;
//         }
//     }

//     vec![duplicate, missing]
// }

use std::collections::HashSet;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let nums_hashset: HashSet<i32> = HashSet::from_iter(nums.clone());
    let hashset_sum = nums_hashset.iter().sum::<i32>();
    let nums_sum = nums.iter().sum::<i32>();
    let all_range_sum = (1..=nums.len() as i32).sum::<i32>();

    let missing = all_range_sum - hashset_sum;
    let duplicate = nums_sum - hashset_sum;
    result.push(duplicate);
    result.push(missing);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
