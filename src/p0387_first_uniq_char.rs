// https://leetcode.com/problems/first-unique-character-in-a-string/

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

#[cfg(test)]
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
