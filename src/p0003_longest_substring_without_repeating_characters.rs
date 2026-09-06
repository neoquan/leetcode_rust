// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut last_seen: HashMap<char, usize> = HashMap::new();

    let mut left: usize = 0;
    let mut max_len: usize = 0;

    for right in 0..chars.len() {
        let current_char = chars[right];
        if let Some(&prev_index) = last_seen.get(&current_char) {
            if prev_index >= left {
                left = prev_index + 1;
            }
        }

        last_seen.insert(current_char, right);

        println!("In right index {right}, the hashmap last_seen is {last_seen:?}");

        let window_len = right - left + 1;

        if window_len > max_len {
            max_len = window_len;
        }
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
