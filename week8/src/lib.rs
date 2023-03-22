/* library for Strobogrammatic Number Determiner */

/* build a function that check if a input number is a strobogrammatic number */

// reference: https://leetcode.com/problems/strobogrammatic-number/solutions/1101463/rust-onepass/?q=rust&orderBy=most_relevant
pub fn is_strobogrammatic(num: String) -> bool {
    let chars: Vec<char> = num.chars().collect();
    let len = chars.len();

    for i in 0..(len / 2) {
        match chars[i] {
            '1' | '8' | '0' => {
                if chars[i] != chars[len - i - 1] {
                    return false;
                }
            }
            '6' => {
                if chars[len - i - 1] != '9' {
                    return false;
                }
            }
            '9' => {
                if chars[len - i - 1] != '6' {
                    return false;
                }
            }
            _ => return false,
        }
    }
    if len % 2 == 1 {
        let mid_char = chars[len / 2];
        if mid_char != '1' && mid_char != '8' && mid_char != '0' {
            return false;
        }
    }
    true
}
