// https://leetcode.com/problems/valid-anagram/description/

// use std::collections::HashMap;

// pub fn is_anagram(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false
//     }
//     let mut counts: HashMap<char, i32> = HashMap::new();
//     let mut countt: HashMap<char, i32> = HashMap::new();
//     for c in s.chars() {
//         *counts.entry(c).or_insert(0) += 1;
//     }
//     for c in t.chars() {
//         *countt.entry(c).or_insert(0) += 1;
//     }
//     counts == countt
// }

// pub fn is_anagram(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }
//     let mut counts: HashMap<char, i32> = HashMap::new();
//     for c in s.chars() {
//         *counts.entry(c).or_insert(0) += 1;
//     }
//     for c in t.chars() {
//         *counts.entry(c).or_insert(0) -= 1;
//     }
//     counts.values().all(|&v| v == 0) // return Boolean
// }

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = [0i32; 26];
    for (bs, bt) in s.bytes().zip(t.bytes()) {
        counts[(bs - b'a') as usize] += 1;
        counts[(bt - b'a') as usize] -= 1;
    }
    counts.iter().all(|&c| c == 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}