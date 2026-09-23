// https://leetcode.com/problems/online-stock-span/

struct StockSpanner {
    stack: Vec<(i32, i32)>, // (price, span) — persists across next() calls
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: Vec::new() } // start empty
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1; // today counts as itself
        // pop every previous day whose price ≤ today's, absorbing its span
        while let Some(&(top_price, top_span)) = self.stack.last() {
            if top_price <= price {
                span += top_span; // ← absorb-on-pop: grab the cached count
                self.stack.pop();
            } else {
                break;
            }
        }
        self.stack.push((price, span));
        span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // the LeetCode example: prices 100,80,60,70,60,75,85 → spans 1,1,1,2,1,4,6
        let mut s = StockSpanner::new();
        assert_eq!(s.next(100), 1);
        assert_eq!(s.next(80), 1);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(70), 2);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(75), 4);
        assert_eq!(s.next(85), 6);
    }

    #[test]
    fn single_day() {
        let mut s = StockSpanner::new();
        assert_eq!(s.next(42), 1); // first day is always span 1
    }

    #[test]
    fn strictly_increasing() {
        // each day ≥ all before → absorbs the entire history, span grows 1,2,3,4,5
        let mut s = StockSpanner::new();
        for (i, p) in [10, 20, 30, 40, 50].into_iter().enumerate() {
            assert_eq!(s.next(p), (i + 1) as i32);
        }
    }

    #[test]
    fn strictly_decreasing() {
        // every day is blocked immediately by the taller day before it → all spans 1
        let mut s = StockSpanner::new();
        for p in [50, 40, 30, 20, 10] {
            assert_eq!(s.next(p), 1);
        }
    }

    #[test]
    fn all_equal() {
        // equal counts as ≤, so each day absorbs all previous → 1,2,3,4
        let mut s = StockSpanner::new();
        for (i, p) in [7, 7, 7, 7].into_iter().enumerate() {
            assert_eq!(s.next(p), (i + 1) as i32);
        }
    }
}
