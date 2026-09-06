// https://leetcode.com/problems/range-sum-query-immutable/


struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_vec = vec![0i32; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_vec[i + 1] = prefix_vec[i] + nums[i];
        }
        NumArray { prefix: prefix_vec }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[right as usize + 1] - self.prefix[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        // [-2, 0, 3, -5, 2, -1] is the example from the problem
        let arr = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);

        assert_eq!(arr.sum_range(0, 2), 1);   // -2 + 0 + 3
        assert_eq!(arr.sum_range(2, 5), -1);  // 3 + -5 + 2 + -1
        assert_eq!(arr.sum_range(0, 5), -3);  // whole array
    }

    #[test]
    fn single_element() {
        let arr = NumArray::new(vec![5]);
        assert_eq!(arr.sum_range(0, 0), 5);   // left == right, one element
    }

    #[test]
    fn full_range() {
        let arr = NumArray::new(vec![1, 2, 3, 4]);
        assert_eq!(arr.sum_range(0, 3), 10);
    }
}
