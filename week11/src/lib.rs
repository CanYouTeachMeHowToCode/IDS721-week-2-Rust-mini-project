/* library for Unix Path Simplifier */

/* build a function that converts an absolute path to a file/directory in a Unix-style file system to the simplified cononical path */

// reference: https://leetcode.com/problems/simplify-path/solutions/1671723/rust/?q=Rust&orderBy=most_relevant

pub fn simplify_path(path: String) -> String {
    format!(
        "/{}",
        path.split('/')
            .filter(|&x| !x.is_empty() && x != ".")
            .fold(vec![], |mut acc, item| {
                if item == ".." {
                    acc.pop();
                } else {
                    acc.push(item);
                }
                acc
            })
            .join("/")
    )
}
