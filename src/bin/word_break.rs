use std::collections::HashSet;

pub fn main() {
    println!("Word Break");
}

// O(nm) Top Down
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];

    dp[0] = true;

    for i in 1..=s.len() {
        for word in word_dict.iter() {
            if word.len() > i {
                continue;
            }

            let slice = &s[(i - word.len())..i];

            if dp[i - word.len()] && slice == word {
                dp[i] = true;
                break;
            }
        }
    }

    dp[s.len()]
}

// O(nm) Bottom Up
pub fn word_break_bottom_up(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];

    dp[s.len()] = true;

    for i in (0..=s.len()).rev() {
        for word in word_dict.iter() {
            if (i + word.len()) <= s.len() && &s[i..i + word.len()] == word {
                dp[i] = dp[i + word.len()];
                if dp[i] {
                    break;
                }
            }
        }
    }

    dp[0]
}

// O(nm) Top Down HashSet
pub fn word_break_hash(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];

    let dict: HashSet<String> = word_dict.into_iter().collect();

    dp[0] = true;

    for i in 1..=s.len() {
        for j in (0..=i - 1).rev() {
            if dp[j] && dict.contains(&s[j..=i - 1]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[s.len()]
}

// O(nm) HashSet cleaner not efficient
pub fn word_break_hash_2(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];

    dp[0] = true;

    let dict: HashSet<String> = HashSet::from_iter(word_dict);

    for start in 0..s.len() {
        for end in start + 1..=s.len() {
            if dict.contains(&s[start..end]) && dp[start] {
                dp[end] = true;
            }
        }
    }

    dp[s.len()]
}
