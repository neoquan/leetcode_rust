// https://leetcode.com/problems/find-all-duplicates-in-an-array/description/

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut counts = vec![0i32; n + 1];

    for &i in &nums {
        counts[i as usize] += 1;
    }

    // println!("For nums = {nums:?}, counts is {counts:?}");

    (1..=n)
        .filter(|&i| counts[i] == 2)
        .map(|i| i as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
    }

    #[test]
    fn example_3() {
        assert_eq!(find_duplicates(vec![1]), vec![]);
    }
}
