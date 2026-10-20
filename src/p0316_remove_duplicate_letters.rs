// https://leetcode.com/problems/remove-duplicate-letters/description/?envType=problem-list-v2&envId=monotonic-stack
//
// 316. Remove Duplicate Letters  (same as 1081. Smallest Subsequence of Distinct Characters)
// Pattern family: monotonic stack (greedy, with a guard)
//
// ============================================================================
// GUIDELINE
// ============================================================================
//
// 1. RECOGNIZE THE PATTERN
//    - "smallest lexicographic"      -> small letters as early as possible
//                                       -> pop bigger letters off the end
//    - keep original order           -> build left to right, only delete
//    - constraint on deletion        -> popping needs a GUARD
//      (each letter exactly once)       (only pop if it's safe)
//
//    Plain monotonic stack:  pop while new element beats top
//    This problem:           pop while new element beats top AND the guard allows it
//
// 2. WHAT DO I NEED IN O(1)?
//    "Is this letter already in the result?"  -> in_stack[26]  (bool marker)
//    "Will top appear again later?"           -> last[26]      (last index)
//    "What did I keep most recently?"         -> stack: Vec<u8>
//
// 3. MAIN LOOP: three decisions per character
//    for each (i, c):
//      1. SKIP  if c already in stack
//      2. POP   while top > c AND top appears after i
//               -> pop, clear top's marker
//      3. PUSH  c, set c's marker
//
// 4. INVARIANTS (why it's correct)
//    - The stack holds the best prefix so far; each letter appears at most once.
//    - Pop only when it's free: a bigger top with a later copy can be picked
//      up again, so removing it now costs nothing and shrinks the prefix.
//    - Forced keep: if top has no later copy, it stays even if it's bigger.
//      Coverage beats smallness.
//    - Skip is safe: everything above an existing c in the stack is bigger
//      than c (anything smaller would have popped it), so the old c is
//      already in its best position.
//
// 5. TRACE: "cbacdcbc"   last: a=2, b=6, c=7, d=4
//
//    i  ch  action                                       stack
//    0  c   push                                         [c]
//    1  b   b<c, last[c]=7 > 1 -> pop c, push b          [b]
//    2  a   a<b, last[b]=6 > 2 -> pop b, push a          [a]
//    3  c   c>a -> push                                  [a, c]
//    4  d   d>c -> push                                  [a, c, d]
//    5  c   in_stack[c] -> SKIP                          [a, c, d]
//    6  b   b<d, but last[d]=4 < 6 -> forced keep, push  [a, c, d, b]
//    7  c   in_stack[c] -> SKIP                          [a, c, d, b]
//    result: "acdb"
//
// 6. PITFALLS (my friction log)
//    - last[x] += 1      -> counts occurrences; need last[x] = i (last index)
//    - top_idx after the while-let block -> out of scope (let lives until
//      its block's closing brace); the current letter is idx
//    - forgot to clear the marker on pop -> letter skipped forever, missing
//      from the result
//    - marker arrays that can be UNSET need both directions:
//      set on push, clear on pop
//    - no final expression -> function returns () instead of String
//
// 7. COMPLEXITY
//    Time  O(n): each char pushed at most once, popped at most once
//    Space O(1): last, in_stack, and stack are all capped at 26
//
// 8. SAME TEMPLATE, DIFFERENT GUARD (practice next)
//    1081 Smallest Subsequence of Distinct Characters -> identical
//    402  Remove K Digits                -> guard: pops left < k
//    1673 Find the Most Competitive Subsequence -> guard: enough elements left
//    321  Create Maximum Number          -> harder: twice + merge
//
//    Template: pop while (new beats top) AND (guard says popping is safe)
// ============================================================================

pub fn remove_duplicate_letters(s: String) -> String {
    let bytes = s.as_bytes();

    // Pass 1: last index of each letter. Overwrite on every hit; the later write wins.
    let mut last = [0usize; 26];
    for (i, &b) in bytes.iter().enumerate() {
        last[(b - b'a') as usize] = i;
    }

    println!("With s is {s}, 'last' array is {last:?}");

    let mut stack: Vec<u8> = Vec::with_capacity(26);
    let mut in_stack = [false; 26];

    // Pass 2: greedy monotonic stack
    for (i, &b) in bytes.iter().enumerate() {
        let idx = (b - b'a') as usize;

        // Decision 1: already in the result -> skip
        if in_stack[idx] {
            continue;
        }

        // Decision 2: pop while top is bigger AND top appears again later
        while let Some(&top) = stack.last() {
            let top_idx = (top - b'a') as usize;
            if idx < top_idx && i < last[top_idx] {
                stack.pop();
                in_stack[top_idx] = false; // top left -> clear ITS marker
            } else {
                break; // top is smaller, or this is its last chance
            }
            // println!("With i = {i}, 'stack' array is {stack:?} while 'in_stack' array is {in_stack:?}");
        }

        // Decision 3: push current, mark it
        stack.push(b);
        in_stack[idx] = true;
    }

    // Vec<u8> -> String without Result/unwrap: u8 -> char -> collect
    stack.iter().map(|&b| b as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(remove_duplicate_letters("bcabc".to_string()), "abc");
    }

    // #[test]
    // fn example_2() {
    //     // exercises pop, forced keep, and skip
    //     assert_eq!(remove_duplicate_letters("cbacdcbc".to_string()), "acdb");
    // }

    // #[test]
    // fn single_char() {
    //     assert_eq!(remove_duplicate_letters("a".to_string()), "a");
    // }

    // #[test]
    // fn already_unique() {
    //     // nothing can be popped: every letter is its own last copy
    //     assert_eq!(remove_duplicate_letters("dcba".to_string()), "dcba");
    // }
}
