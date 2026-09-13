// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/?envType=problem-list-v2&envId=dsa-linear-shoal-array-ii

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    // build historgram
    let n = nums.len();
    let mut count = vec![0i32; n + 1];
    for &i in &nums {
        count[i as usize] += 1;
    }

    // println!("The histogram count is : {count:?}");

    let mut result = Vec::new();

    for v in 1..=n {
        if count[v] == 0 {
            result.push(v as i32);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
