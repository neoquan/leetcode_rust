// https://leetcode.com/problems/first-unique-character-in-a-string/
    

// use std::collections::HashMap;
// pub fn first_uniq_char(s: String) -> i32 {
//     let mut storage: HashMap<char, i32> = HashMap::new();

//     for c in s.chars() {
//         *storage.entry(c).or_insert(0) += 1;
//     }
//     for (index, c) in s.chars().enumerate() {
//         match storage.get(&c) {
//             Some(count) if *count == 1 => return index as i32,
//             _ => {}
//         }
//     }
//     -1
// }


pub fn first_uniq_char(s: String) -> i32 {
    let mut counts = [0i32; 26];
    let bytes = s.as_bytes();

    for &byte in bytes {
        counts[(byte - b'a') as usize] += 1;
    }

    for (index, &byte) in bytes.iter().enumerate() {
        if counts[(byte - b'a') as usize] == 1 {
            return index as i32;
        }
    }
    -1
}

mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(first_uniq_char("aabb".to_string()), -1);
    }
}