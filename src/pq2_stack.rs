// https://leetcode.com/problems/evaluate-reverse-polish-notation/description/?envType=problem-list-v2&envId=dsa-linear-shoal-stack

use std::str::FromStr;


enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Token {
    Num(i32),
    Operator(Op),
}

impl FromStr for Token {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = match s {
            "+" => Token::Operator(Op::Add),
            "-" => Token::Operator(Op::Sub),
            "*" => Token::Operator(Op::Mul),
            "/" => Token::Operator(Op::Div),
            _   => Token::Num(s.parse::<i32>()?),
        };
        Ok(token)
    }
}


pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for t in tokens {
        let tok = match t.parse::<Token>() {
            Ok(tok) => tok,
            Err(_)  => continue,
        };
        match tok {
            Token::Num(n) => stack.push(n),
            Token::Operator(op) => {
                let b = match stack.pop() {      // top of stack → RIGHT operand
                    Some(v) => v,
                    None    => continue,
                };
                let a = match stack.pop() {      // next → LEFT operand
                    Some(v) => v,
                    None    => continue,
                };

                let r = match op {
                    Op::Add => a + b,
                    Op::Sub => a - b,
                    Op::Mul => a * b,
                    Op::Div => a / b,
                };

                stack.push(r);
            }
        }
    }
    match stack.pop() {
        Some(v) => v,
        None    => 0,
    }
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

    #[test]
    fn example_2() {
        assert_eq!(
            eval_rpn(vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()]), 6
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            eval_rpn(vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),
            "-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),
            "+".to_string(),"5".to_string(),"+".to_string()]), 22
        );
    }
}
