// https://leetcode.com/problems/largest-rectangle-in-histogram/description/?envType=problem-list-v2&envId=dsa-linear-shoal-monotonic-stack


// monotonic-stack version

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut best: i32 = 0;

    for i in 0..=n {
        let cur = if i == n { 0 } else { heights[i] };

        // println!("For i = {i}: stack = {stack:?}");

        while let Some(&top) = stack.last() {
            if heights[top] > cur {
                stack.pop();
                let height = heights[top];
                let width = match stack.last() {
                    Some(&left) => i - left - 1,
                    None => i,
                };
                best = best.max(height * width as i32);
                // println!("For i = {i}: top = {top}, height = {height}, width = {width}, best = {best}");
            } else {
                break;
            }
        }
        stack.push(i);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
    }

    // #[test]
    // fn example_2() {
    //     assert_eq!(largest_rectangle_area(vec![30,40,50,60]), vec![1,1,1,0]);
    // }

}
