use std::collections::HashSet;

pub fn main() {
    println!("Longest Commen Subsequence");
}

// Bottom up O(mn)
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    let text1: Vec<char> = text1.chars().collect();
    let text2: Vec<char> = text2.chars().collect();

    for i in (0..text1.len()).rev() {
        for j in (0..text2.len()).rev() {
            if text1[i] == text2[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}
