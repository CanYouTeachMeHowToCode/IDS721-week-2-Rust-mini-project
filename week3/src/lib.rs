/* A library for Basic Calculator */

/* build a function that takes in a string that represents a mathematical expression
that contains numbers (fit in 32-bit integers) and '+', '-', '*', and '/' symbols
(e.g. 2*3+4/5), evaluate this expression and returns its value.
Note: the integer division here will truncate toward 0. */

pub fn int_to_roman(mut num: i32) -> String {
    let mut res = String::new();
    let values = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    for (letter, value) in values.iter() {
        while num >= *value {
            res.push_str(letter);
            num -= *value;
        }
    }
    res
}
