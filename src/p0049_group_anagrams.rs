// https://leetcode.com/problems/group-anagrams/description/

use std::collections::HashMap;

// 
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut storage: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0i32; 26];
        for &b in s.as_bytes() {
            key[(b - b'a') as usize] += 1;
        }
        storage.entry(key).or_insert_with(Vec::new).push(s);
    }

    storage.into_values().collect()
}

fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for g in &mut groups {
        g.sort();               // sort within each group
    }
    groups.sort();              // then sort the groups themselves
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let got = group_anagrams(vec!["eat".to_string(), "tea".to_string(),
                                    "tan".to_string(), "ate".to_string(),
                                    "nat".to_string(), "bat".to_string()]);
        let want = vec![vec!["bat".to_string()],
                        vec!["nat".to_string(), "tan".to_string()],
                        vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]];
        assert_eq!(normalize(got), normalize(want));
    }
}