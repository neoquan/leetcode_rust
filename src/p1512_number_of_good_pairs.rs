// https://leetcode.com/problems/number-of-good-pairs/


use std::collections::HashMap;


pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut storage: HashMap<i32, i32> = HashMap::new();
    let mut result: i32 = 0;
    for i in nums {
        *storage.entry(i).or_insert(0) += 1;
    }
    for v in storage.values() {
        result += (v * (v - 1)) / 2;
    }
    result
}


mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(num_identical_pairs(vec![1,2,3,1,1,3]), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(num_identical_pairs(vec![1,1,1,1]), 6);
    }

    #[test]
    fn example_3() {
        assert_eq!(num_identical_pairs(vec![1,2,3]), 0);
    }
}