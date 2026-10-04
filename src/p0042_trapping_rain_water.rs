// https://leetcode.com/problems/trapping-rain-water/

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    };

    let mut left_max: Vec<i32> = vec![0; n];
    let mut right_max: Vec<i32> = vec![0; n];

    // pass 1: left→right. each cell is the running max including itself.

    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    // pass 2: right→left. same idea, other direction.

    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        // n-2, n-3, ..., 0
        right_max[i] = right_max[i + 1].max(height[i]);
    }

    // pass 3: sum the water sitting on each position.
    let mut total = 0;
    for i in 0..n {
        total += left_max[i].min(right_max[i]) - height[i]; // C: water on top of bar i
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
