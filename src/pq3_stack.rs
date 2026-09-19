// https://leetcode.com/problems/exclusive-time-of-functions/?envType=problem-list-v2&envId=dsa-linear-shoal-stack

pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut result = vec![0; n as usize];
    let mut stack: Vec<usize> = Vec::new();
    let mut prev: i32 = 0;

    for log in &logs {
        let mut parts = log.split(":");
        let id = match parts.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => continue,
        };

        let kind = match parts.next() {
            Some(v) => v,
            None => continue,
        };

        let time = match parts.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => continue,
        };

        if kind == "start" {
            if let Some(&top) = stack.last() {
                result[top] += time - prev;
            }
            stack.push(id);
            prev = time;
        } else {
            if let Some(top) = stack.pop() {
                result[top] += time - prev + 1;
            }
            prev = time + 1;
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
            exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "1:start:2".to_string(),
                    "1:end:5".to_string(),
                    "0:end:6".to_string()
                ]
            ),
            vec![3, 4]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            exclusive_time(
                1,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "0:start:6".to_string(),
                    "0:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![8]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:6".to_string(),
                    "1:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![7, 1]
        );
    }
}
