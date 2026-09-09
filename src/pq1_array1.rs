// https://leetcode.com/problems/concatenation-of-array/description/?envType=problem-list-v2&envId=dsa-linear-shoal-array-i

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(2 * nums.len());
    ans.extend_from_slice(&nums);
    ans.extend_from_slice(&nums);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
