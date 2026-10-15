// https://leetcode.com/problems/find-the-duplicate-number/description/

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let n: usize = nums.len();
    let mut counts = vec![0i32; n + 1];

    for &i in &nums {
        counts[i as usize] += 1;
    }

    match (1..n).find(|&i| counts[i] > 1) {
        Some(i) => i as i32,
        None => -1, // unreachable given the problem's guarantee
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(find_duplicate(vec![3, 3, 3, 3, 3]), 3);
    }
}
