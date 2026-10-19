// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/description/

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = Vec::new();
    let mut result: Vec<i32> = prices.clone(); // default: no discount

    for i in 0..prices.len() {
        while let Some(&top) = stack.last() {
            if prices[i] <= prices[top] {
                result[top] = prices[top] - prices[i]; // write to its own slot
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
    }

    #[test]
    fn example_2() {
        assert_eq!(final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example_3() {
        assert_eq!(final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
}
