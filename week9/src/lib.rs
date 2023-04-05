/* library for Gray Code Generator */

/* build a function that generates the gray code of the input integer */

// reference: https://leetcode.com/problems/gray-code/solutions/717330/rust-solution/?q=rust&orderBy=most_relevant
pub fn gray_code(n: i32) -> Vec<i32> {
    let mut x = 1;
    let mut res = vec![0];

    for _ in 0..n {
        let mut rev = res.iter().rev().map(|&num| num + x).collect();
        res.append(&mut rev);
        x *= 2;
    }

    res
}
