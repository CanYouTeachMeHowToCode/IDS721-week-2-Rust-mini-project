/* library for Invalid Parentheses Remover */

/* build a function that generates all the valid results of an input String of parentheses after removal  */

// reference: https://leetcode.com/problems/remove-invalid-parentheses/solutions/864630/rust-translated-0ms-100/?q=Rust&orderBy=most_relevant
pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    fn remove(s: &[char], res: &mut Vec<String>, last_i: i32, last_j: i32, par: &[char]) {
        let mut stack = 0;
        for i in last_i..s.len() as i32 {
            if s[i as usize] == par[0] {
                stack += 1;
            }
            if s[i as usize] == par[1] {
                stack -= 1;
            }
            if stack >= 0 {
                continue;
            }
            for j in last_j..=i {
                if s[j as usize] == par[1] && (j == last_j || s[j as usize - 1] != par[1]) {
                    let mut s2 = Vec::<char>::new();
                    for curr in s.iter().take(j as usize) {
                        s2.push(*curr);
                    }
                    for curr in s.iter().skip(j as usize + 1) {
                        s2.push(*curr);
                    }
                    remove(&s2, res, i, j, par);
                }
            }
            return;
        }
        let mut reversed = s.to_vec();
        reversed.reverse();
        if par[0] == '(' {
            remove(&reversed, res, 0, 0, &[')', '(']);
        } else {
            res.push(String::from_iter(reversed));
        }
    }
    let mut res = vec![];
    remove(
        &s.chars().collect::<Vec<char>>(),
        &mut res,
        0,
        0,
        &['(', ')'],
    );
    res
}
