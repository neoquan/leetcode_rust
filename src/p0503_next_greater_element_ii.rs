// https://leetcode.com/problems/next-greater-element-ii/description/

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result: Vec<i32> = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..2 * n - 1 {
        let cur = nums[i % n];
        while let Some(&top) = stack.last() {
            if cur > nums[top] {
                result[top] = cur;
                stack.pop();
            } else {
                break;
            }
        }
        if i < n {
            stack.push(i % n);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }
}
