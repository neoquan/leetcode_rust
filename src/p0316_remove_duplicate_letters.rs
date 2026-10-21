// https://leetcode.com/problems/remove-duplicate-letters/description/?envType=problem-list-v2&envId=monotonic-stack
//
// 316. Remove Duplicate Letters  (same as 1081. Smallest Subsequence of Distinct Characters)
// Pattern family: monotonic stack (greedy, with a guard)
//
// ============================================================================
// GUIDELINE
// ============================================================================
//
// 0. THE IDEA (why the solution looks the way it does, not how to code it)
//
//    Three intuitions. If you believe these three, the algorithm is forced;
//    you are not inventing it, you are just obeying them.
//
//    IDEA 1 -- The earliest slot is everything.
//      Lexicographic order is read left to right and the first difference
//      decides the whole comparison. "az" < "b..." no matter what follows.
//      So a smaller letter sitting one position earlier is worth more than
//      anything you could ever gain later. The result is therefore: make
//      the first letter as small as you can afford, then the second, and so
//      on. That single fact is WHY the approach is greedy left-to-right and
//      not some global search -- early wins dominate late wins absolutely.
//
//    IDEA 2 -- A letter that appears again later is a "free undo".
//      You are walking the string and have already committed some letters.
//      You now meet a letter smaller than one you kept. Do you regret the
//      bigger one? Only if you can safely get rid of it. And you CAN get rid
//      of it for free precisely when it occurs again further ahead -- because
//      then you haven't truly lost it; its later copy will let you place it
//      back wherever it belongs. A future occurrence is a safety net that
//      turns "evict the bigger letter to make room for this smaller one"
//      into a move with zero downside.
//
//    IDEA 3 -- A letter with no copy ahead is load-bearing; it must stay.
//      The result must still contain every distinct letter. So if the bigger
//      letter you're tempted to drop never shows up again, dropping it makes
//      the answer invalid -- there is no later copy to recover it from. Here
//      coverage beats smallness: you are forced to keep it even though a
//      smaller letter is knocking. This is the "guard" that separates this
//      problem from a plain greedy -- the regret from IDEA 2 is only
//      actionable when IDEA 3 permits it.
//
//    Put together: scan left to right; whenever a small letter arrives, undo
//    as many earlier-but-bigger commitments as are safe to undo (IDEA 2),
//    stopping at any that are load-bearing (IDEA 3), because each undo buys a
//    smaller letter in an earlier slot and earlier slots are what win
//    (IDEA 1). "Undo the most recent commitments" is literally a stack --
//    which is why the mechanics below are a monotonic stack with a guard.
//    And a letter already kept needs no action: everything above it is
//    bigger (or it would have been undone), so it already sits in its best
//    possible slot -- nothing to gain by moving it.
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
    // Trace helper: render a byte stack as readable letters.
    // (A bare `{stack:?}` on Vec<u8> prints ASCII codes like [97, 99], not "ac".)
    let show = |st: &[u8]| -> String { st.iter().map(|&b| b as char).collect() };

    // One line per character; action is padded so the `stack=` column lines up.
    let step = |i: usize, c: char, action: &str, st: &[u8]| {
        println!("i={i} ch={c}  {action:<30} stack={}", show(st));
    };

    let bytes = s.as_bytes();

    // Pass 1: last index of each letter. Overwrite on every hit; the later write wins.
    let mut last = [0usize; 26];
    for (i, &b) in bytes.iter().enumerate() {
        last[(b - b'a') as usize] = i;
    }

    println!("\ninput = {s:?}");
    print!("last-seen index: ");
    for (i, &b) in bytes.iter().enumerate() {
        if last[(b - b'a') as usize] == i {
            print!("{c}={i} ", c = b as char); // print each letter once, at its last index
        }
    }
    println!("\n{:-<60}", "");

    let mut stack: Vec<u8> = Vec::with_capacity(26);
    let mut in_stack = [false; 26];

    // Pass 2: greedy monotonic stack
    for (i, &b) in bytes.iter().enumerate() {
        let c = b as char;
        let idx = (b - b'a') as usize;

        // Decision 1: already in the result -> skip
        if in_stack[idx] {
            step(i, c, "SKIP (already kept)", &stack);
            continue;
        }

        // Decision 2: pop while top is bigger AND top appears again later
        while let Some(&top) = stack.last() {
            let top_idx = (top - b'a') as usize;
            if idx < top_idx && i < last[top_idx] {
                stack.pop();
                in_stack[top_idx] = false; // top left -> clear ITS marker
                step(i, c, &format!("pop '{}' (bigger, recurs later)", top as char), &stack);
            } else {
                if idx < top_idx {
                    // bigger top, but this is its last copy -> coverage forces the keep
                    step(i, c, &format!("keep '{}' (bigger, but LAST copy)", top as char), &stack);
                }
                break; // top is smaller, or this is its last chance
            }
        }

        // Decision 3: push current, mark it
        stack.push(b);
        in_stack[idx] = true;
        step(i, c, "push", &stack);
    }

    let result: String = stack.iter().map(|&b| b as char).collect();
    println!("{:-<60}\nresult = {result:?}\n", "");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn example_1() {
    //     assert_eq!(remove_duplicate_letters("bcabc".to_string()), "abc");
    // }

    #[test]
    fn example_2() {
        // exercises pop, forced keep, and skip
        assert_eq!(remove_duplicate_letters("cbacdcbc".to_string()), "acdb");
    }

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
