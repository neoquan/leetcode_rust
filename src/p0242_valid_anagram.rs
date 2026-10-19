// https://leetcode.com/problems/valid-anagram/description/

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
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
