// https://leetcode.com/problems/find-and-replace-pattern/description/

use std::collections::HashMap;

fn normalize(word: &str) -> Vec<u8> {
    let mut seen: HashMap<u8, u8> = HashMap::new();
    let mut counter: u8 = 0;
    let mut sig: Vec<u8> = Vec::with_capacity(word.len());

    for i in word.as_bytes() {
        let id = match seen.get(&i) {
            Some(&existing) => existing,
            None => {
                let new_id = counter;
                seen.insert(*i, new_id);
                counter += 1;
                new_id
            }
        };
        sig.push(id);
    }
    sig
}

pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    let target = normalize(&pattern);
    words
        .into_iter()
        .filter(|w| normalize(w) == target)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            find_and_replace_pattern(
                vec![
                    "abc".to_string(),
                    "deq".to_string(),
                    "mee".to_string(),
                    "aqq".to_string(),
                    "dkd".to_string(),
                    "ccc".to_string()
                ],
                "abb".to_string()
            ),
            vec!["mee".to_string(), "aqq".to_string()]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            find_and_replace_pattern(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "a".to_string()
            ),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
