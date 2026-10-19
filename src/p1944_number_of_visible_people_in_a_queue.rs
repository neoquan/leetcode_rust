// https://leetcode.com/problems/number-of-visible-people-in-a-queue/

pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let n = heights.len();
    let mut result: Vec<i32> = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in (0..n).rev() {
        let cur = heights[i];
        while let Some(&top) = stack.last() {
            result[i] += 1;
            if cur > heights[top] {
                result[i] += 1;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            vec![3, 1, 2, 1, 1, 0]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            can_see_persons_count(vec![5, 1, 2, 3, 10]),
            vec![4, 1, 1, 1, 0]
        );
    }
}
