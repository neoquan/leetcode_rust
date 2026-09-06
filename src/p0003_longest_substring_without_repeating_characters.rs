// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

use std::collections::HashMap;

// The idea is to use 2 techniques: two pointer and sliding windows
// Firstly, we create hashmap to store: previously visited index (cur_char)
// we keep left pointer = 0 initially, and then iterate right through chars.
// save current char as chars[right]
// if that cur_char exist in hashmap => then compare value of that key cur_char with left pointer
// if higher or equal, we increment left pointer by +1 to prev_index
// We also need to track windows length; if its longer than maximum length => update max_length

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut storage: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();

    let mut left: usize = 0;
    let mut max_len: usize = 0;

    for right in 0..chars.len() {
        let cur_char = chars[right];

        if let Some(&prev_index) = storage.get(&cur_char) {
            if prev_index >= left {
                left = prev_index + 1;
            }
        }

        storage.insert(cur_char, right);

        println!("In right index {right}, the hashmap storage is {storage:?}");

        let window_len= right - left + 1;

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
