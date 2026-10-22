// https://leetcode.com/problems/remove-k-digits/description/?envType=problem-list-v2&envId=monotonic-stack

pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k as usize;
    let mut stack: Vec<u8> = Vec::with_capacity(num.len());

    for b in num.bytes() {
        // pop while: k left AND stack has a top AND top is bigger than b
        while k > 0 && matches!(stack.last(), Some(&top) if top > b) {
            stack.pop();
            k -= 1;
        }
        stack.push(b);
    }

    // 1. still owe removals? chop from the end
    stack.truncate(stack.len() - k);

    // println!("Stack after truncating is {stack:?}");
    // 2. skip leading b'0's — find first non-zero index
    let start = stack.iter().position(|&b| b != b'0').unwrap_or(stack.len());

    // println!("start value is {start}");

    // 3. empty → "0", else bytes → String
    if start == stack.len() {
        "0".to_string()
    } else {
        // stack[start..] is all ASCII digits, so this can't fail;
        // match on String::from_utf8(...) instead of unwrap
        match String::from_utf8(stack[start..].to_vec()) {
            Ok(s) => s,
            Err(_) => "0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(remove_kdigits("1432219".to_string(), 3), "1219".to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(remove_kdigits("10200".to_string(), 1), "200".to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(remove_kdigits("10".to_string(), 2), "0".to_string());
    }
}
