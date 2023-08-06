use std::{collections::HashSet, str::from_utf8};

pub fn main() {
    println!("Longest Common Prefix");
}

// My solution
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let (first_word, rest) = strs.split_at(1);

    let mut prefix = first_word[0].clone();

    for word in rest {
        while !word.starts_with(&prefix) {
            prefix.pop().unwrap();

            if prefix.len() == 0 {
                return String::default();
            }
        }
    }

    return prefix;
}

// Using iter methods
pub fn longest_common_prefix_iter(strs: Vec<String>) -> String {
    strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
        acc.chars()
            .zip(x.chars())
            .take_while(|(ch_1, ch_2)| ch_1 == ch_2)
            .map(|(ch_1, _)| ch_1)
            .collect()
    })
}

// This will give the longest prefix between any two words
pub fn longest_common_prefix_wrong(strs: Vec<String>) -> String {
    let mut all_possibilites = HashSet::new();
    let mut prefixes = HashSet::new();

    for word in strs.iter() {
        let word = word.as_bytes();

        for i in 0..word.len() {
            if !all_possibilites.insert(&word[..i]) {
                prefixes.insert(&word[..i]);
            }
        }
    }

    prefixes
        .iter()
        .map(|slice| (slice, slice.len()))
        .max_by_key(|t| t.1)
        .map(|(bytes, _)| from_utf8(bytes).unwrap().to_string())
        .unwrap_or_else(|| String::default())
}
