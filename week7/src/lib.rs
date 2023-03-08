/* library for Anagram Finder */

/* build a function that find all anagrams in a string */
use std::collections::{HashMap, HashSet};
pub fn find_anagrams(input: String) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut chars: Vec<char> = input.chars().collect();
    let length = chars.len();

    generate_anagrams(&mut chars, length, &mut res);
    // dedup
    let res: HashSet<String> = res.into_iter().collect();
    let mut res: Vec<String> = res.into_iter().collect();
    res.sort();
    res
}

fn generate_anagrams(chars: &mut Vec<char>, length: usize, result: &mut Vec<String>) {
    if length == 1 {
        result.push(chars.iter().collect());
        return;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for &c in chars.iter() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    for i in 0..length {
        let c = chars[i];
        let count = counts.get_mut(&c).unwrap();

        if *count == 0 {
            continue;
        }

        *count -= 1;

        generate_anagrams(chars, length - 1, result);

        *count += 1;
        chars.swap(i, length - 1);
    }
}
