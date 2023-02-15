/* library for Longest Palidromic Substring Finder */

/* build a function that takes in a string and return its longest palindromic substring */

pub fn longest_palindromic_substring(s: &str) -> String {
    if s.is_empty() {
        return String::from("");
    }

    let mut start = 0;
    let mut end = 0;
    let chs: Vec<char> = s.chars().collect();
    let mut f = vec![vec![false; chs.len()]; chs.len()];

    for j in 0..chs.len() {
        let mut i = 0;
        f[j][j] = true;
        while i <= j {
            f[i][j] = chs[j] == chs[i] && (j - i < 2 || j > 0 && f[i + 1][j - 1]);
            if f[i][j] && (j - i + 1) > (end - start) {
                start = i;
                end = j + 1;
            }
            i += 1;
        }
    }
    return chs[start..end].iter().collect::<String>();
}
