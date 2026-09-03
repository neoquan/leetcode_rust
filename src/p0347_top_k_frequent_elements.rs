// https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut storage: HashMap<i32, i32> = HashMap::new();
    let mut result = Vec::new();

    for i in nums {
        *storage.entry(i).or_insert(0) += 1;
    }

    let mut sorted_elements: Vec<(&i32, &i32)> = storage.iter().collect();

    sorted_elements.sort_by(|a, b| b.1.cmp(a.1));

    // println!("sorted_elements = {sorted_elements:?}");

    for j in 0..k {
        result.push(*sorted_elements[j as usize].0);
    }

    // println!("result = {result:?}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    // #[test]
    // fn example_2() {
    //     assert_eq!(product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    // }
}
