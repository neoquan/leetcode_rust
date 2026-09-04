// https://leetcode.com/problems/longest-consecutive-sequence/

use std::collections::HashSet;


// First, turn vector in a Hash Set in order to have unique element
// Second, we initialize 2 trackers: current and length
// Loop through the set, then if (i-1) is not in the set, then i is the run-start => update current
// Then walk upwards (while), if set contains (current + 1), then it is consecutively lined up => update current and length
// Compare result with the length every loop

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    let uniqueset: HashSet<i32> = nums.into_iter().collect();
    // println!("uniqueset = {uniqueset:?}");

    for i in &uniqueset {
        if !uniqueset.contains(&(i - 1)) {
            let mut current = *i;
            let mut length = 1;

            while uniqueset.contains(&(current + 1)) {
                current += 1;
                length += 1;
            }

            result = result.max(length);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }

        #[test]
    fn example_3() {
        assert_eq!(longest_consecutive(vec![1,0,1,2]), 3);
    }
}
