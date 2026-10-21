// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/


// Input: s = "cbacdcbc"
// Output: "acdb"

pub fn smallest_subsequence(s: String) -> String {
    // We will create a 'last' array where each position of each char
    // will store the last index of each char appearing in s.
    let bytes: &[u8] = s.as_bytes();
    let mut last = [0usize; 26];

    for (index, &b) in bytes.iter().enumerate() {
        last[(b - b'a') as usize] = index;
    }

    let mut stack: Vec<u8> = Vec::with_capacity(26);
    // create a deduplicated array: in_stack
    // if that character is already in the in_stack
    // skip it
    let mut in_stack = vec![false; 26];

    for (index, &b) in bytes.iter().enumerate() {
        let idx = (b - b'a') as usize;
        if in_stack[idx] {
            continue
        }

        while let Some(&top) = stack.last() {
            let top_idx = (top - b'a') as usize;
            // If current position (idx) is smaller than stack's top and
            // current index is smaller than last position of character
            if idx < top_idx && index < last[top_idx] {
                stack.pop();
                in_stack[top_idx] = false;
            } else {
                break;
            }
        }

        stack.push(b);
        in_stack[idx] = true;
    }

    stack.iter().map(|&b| b as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(smallest_subsequence("bcabc".to_string()), "abc");
    }

    #[test]
    fn example_2() {
        // exercises pop, forced keep, and skip
        assert_eq!(smallest_subsequence("cbacdcbc".to_string()), "acdb");
    }

}
