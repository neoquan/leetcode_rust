// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/description/?envType=problem-list-v2&envId=dsa-linear-shoal-monotonic-stack

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let mut result: Vec<i32> = Vec::with_capacity(n);

    for i in 0..n {
        let mut discount = 0;
        for j in (i + 1)..n {
            if prices[j] <= prices[i] {
                discount = prices[j];
                break;
            }
        }
        result.push(prices[i] - discount);
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
