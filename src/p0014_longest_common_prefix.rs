// https://leetcode.com/problems/longest-common-prefix/description/?envType=problem-list-v2&envId=array

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
    for col in 0..min_len {
        let refer = strs[0].as_bytes()[col];
        for str in &strs {
            if refer != str.as_bytes()[col] {
                return strs[0][0..col].to_string();
            }
        }
    }
    strs[0][0..min_len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(
            longest_common_prefix(to_vec(&["flower", "flow", "flight"])),
            "fl"
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            longest_common_prefix(to_vec(&["dog", "racecar", "car"])),
            ""
        );
    }

    #[test]
    fn single_string() {
        assert_eq!(longest_common_prefix(to_vec(&["alone"])), "alone");
    }

    #[test]
    fn empty_string_in_list() {
        assert_eq!(longest_common_prefix(to_vec(&["", "abc"])), "");
    }
}
