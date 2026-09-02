// https://leetcode.com/problems/contains-duplicate/description/

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    for n in nums {
        if !seen.insert(n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(contains_duplicate(vec![1,2,3,4]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}