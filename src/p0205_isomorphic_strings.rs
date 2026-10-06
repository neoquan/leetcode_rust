// https://leetcode.com/problems/isomorphic-strings/description/

use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut s_to_t: HashMap<u8, u8> = HashMap::new();
    let mut t_to_s: HashMap<u8, u8> = HashMap::new();

    for (a, b) in s.bytes().zip(t.bytes()) {
        match s_to_t.get(&a) {
            Some(&mapped) => {
                if mapped != b {
                    return false;
                }
            }
            None => {
                s_to_t.insert(a, b);
            }
        }

        match t_to_s.get(&b) {
            Some(&mapped) => {
                if mapped != a {
                    return false;
                }
            }
            None => {
                t_to_s.insert(b, a);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(is_isomorphic("f11".to_string(), "b23".to_string()), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }
}
