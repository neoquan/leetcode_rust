// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/?envType=problem-list-v2&envId=dsa-linear-shoal-array-ii

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    // Build historgram
    let mut count = [0i32; 101];
    for &num in &nums {
        count[num as usize] += 1;
    }

    let mut lt = [0i32; 101]; // lt[v] = how many elements are strictly < v
    let mut running = 0;

    for v in 0..101 {
        lt[v] = running;
        running += count[v];
    }

    // println!("The final lt is {lt:?}");
    nums.iter().map(|&x| lt[x as usize]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
