// https://leetcode.com/problems/max-consecutive-ones/?envType=problem-list-v2&envId=dsa-linear-shoal-array-i

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut current = 0;
    let mut max = 0;

    for x in nums {
        if x == 1 {
            current += 1;
            max = max.max(current);
        } else {
            current = 0;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
