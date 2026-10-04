// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/


pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut counts = vec![0i32; n + 1];   // indices 0..=n; index 0 unused

    for &v in &nums {
        counts[v as usize] += 1;
    }

    (1..=n) 
        .filter(|&i| counts[i] == 0)
        .map(|i| i as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_disappeared_numbers(vec![1,1]), vec![2]);
    }

}
