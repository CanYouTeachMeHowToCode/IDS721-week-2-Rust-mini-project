/* library for EXCEL Title Converter */

/* build a function that takes in an EXCEL title in string format and return
the corresponding column number */

// Reference: https://leetcode.com/problems/excel-sheet-column-number/solutions/1790863/rust-solution/?q=Rust&orderBy=most_relevant

pub fn title_to_column_number(column_title: String) -> i32 {
    column_title
        .to_uppercase()
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            acc + ((c as i32) - ('A' as i32) + 1) * 26i32.pow(i as u32)
        })
}
