// https://leetcode.com/problems/next-greater-element-i/description/

use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut storage: HashMap<i32, i32> = HashMap::new();

    for &i in &nums2 {
        while let Some(&top) = stack.last() {
            if i > top {
                storage.insert(top, i); // storage value which is bigger than key (top)
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i)
    }

    nums1
        .iter()
        .map(|i| *storage.get(i).unwrap_or(&-1)) // missing key ⇒ -1
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
