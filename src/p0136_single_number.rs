// https://leetcode.com/problems/single-number/description/

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0 ;
    for n in nums {
        result ^= n;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(single_number(vec![2,2,1]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(single_number(vec![4,1,2,1,2]), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(single_number(vec![1]), 1);
    }
}