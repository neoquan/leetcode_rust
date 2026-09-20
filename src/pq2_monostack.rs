// https://leetcode.com/problems/daily-temperatures/?envType=problem-list-v2&envId=dsa-linear-shoal-monotonic-stack

// O(n^2) version
// pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//     let n = temperatures.len();
//     let mut result: Vec<i32> = vec![0; n];

//     for i in 0..n {
//         for j in (i+1)..n {
//             if temperatures[j] > temperatures[i] {
//                 result[i] = (j - i) as i32;
//                 break;
//             }
//         }
//     }
//     result
// }

// monotonic-stack version

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut result: Vec<i32> = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        // temps[i] resolves every waiting day it's warmer than
        while let Some(&top) = stack.last() {
            if temperatures[i] > temperatures[top] {
                result[top] = (i - top) as i32;
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
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    }

    #[test]
    fn example_3() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
