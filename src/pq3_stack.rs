// https://leetcode.com/problems/exclusive-time-of-functions/?envType=problem-list-v2&envId=dsa-linear-shoal-stack

pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            eval_rpn(vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()]), 9
        );
    }

    // #[test]
    // fn example_2() {
    //     assert_eq!(
    //         eval_rpn(vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()]), 6
    //     );
    // }

    // #[test]
    // fn example_3() {
    //     assert_eq!(
    //         eval_rpn(vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),
    //         "-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),
    //         "+".to_string(),"5".to_string(),"+".to_string()]), 22
    //     );
    // }
}
