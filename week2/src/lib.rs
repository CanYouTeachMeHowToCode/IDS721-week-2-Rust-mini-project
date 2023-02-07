/* A library for Basic Calculator */

/* build a function that takes in a string that represents a mathematical expression
that contains numbers (fit in 32-bit integers) and '+', '-', '*', and '/' symbols
(e.g. 2*3+4/5), evaluate this expression and returns its value.
Note: the integer division here will truncate toward 0. */

// Reference: https://leetcode.com/problems/basic-calculator-ii/solutions/1646431/rust-solution/?q=rust&orderBy=most_relevant

pub fn calculate(s: String) -> i32 {
    use std::collections::HashMap;

    let mut num = 0;
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    let priorities: HashMap<char, u8> = [('+', 1), ('-', 1), ('*', 2), ('/', 2)]
        .iter()
        .cloned()
        .collect();
    let calc = |a: i32, b: i32, op: char| -> i32 {
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            _ => a / b,
        }
    };

    s.chars().for_each(|c| match c {
        '0'..='9' => num = (10 * num) + c.to_digit(10).unwrap() as i32,
        '+' | '-' | '/' | '*' => {
            nums.push(num);
            while let Some(last_op) = ops.last() {
                if nums.len() <= 1 || priorities[&c] > priorities[last_op] {
                    break;
                }
                let op = ops.pop().unwrap();
                let (b, a) = (nums.pop().unwrap(), nums.pop().unwrap());
                nums.push(calc(a, b, op));
            }

            ops.push(c);
            num = 0;
        }
        _ => (),
    });
    nums.push(num);

    while let Some(op) = ops.pop() {
        let (b, a) = (nums.pop().unwrap(), nums.pop().unwrap());
        nums.push(calc(a, b, op));
    }
    nums[0]
}
