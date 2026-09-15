// https://leetcode.com/problems/build-an-array-with-stack-operations/description/?envType=problem-list-v2&envId=dsa-linear-shoal-stack

// For each number i you read from the stream (1, 2, …, n): always Push it.
// Then, if i is not the number target currently wants, immediately Pop it back off.
// If it is the number target wants, leave it on the stack.

pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut result = Vec::new();
    let mut j = 0;

    for i in 1..=n {
        if j == target.len() {
            break; // stack length is equal to the target, so we stop
        }
        result.push("Push".to_string());

        if target[j] == i {
            j += 1;
        } else {
            result.push("Pop".to_string());
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
            build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            build_array(vec![1, 2, 3], 3),
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            build_array(vec![1, 2], 4),
            vec!["Push".to_string(), "Push".to_string()]
        );
    }
}
