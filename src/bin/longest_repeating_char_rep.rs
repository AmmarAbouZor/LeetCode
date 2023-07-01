pub fn main() {
    println!("Longest Repeating Char Replacement");
}

/// Intuition
///
/// Maintain a sliding window such that the counts of characters other than most frequent character is less than or equal to k.
/// Consider the length of this candidate for k replacements with the length of other sliding windows.
/// Approach
///
/// Keep char counts for the sliding window, growing on the right in every iteration.
/// Maintain max frequency counter, updating it only when a new max is found.
/// When sliding window shrinks due to advancing left (since we cannot satisfy with k replacements),
/// we don't need to update max frequency because we already know the max window result len achievable with replacements on top of current max frequency,
/// hence a decrease in max frequency cannot lead to a larger window. So we wait for next window that has a max frequency counter greater that's greater than current max frequency, to check the update the max sliding window length.

// O(n) Space O(1)
pub fn character_replacement(s: String, k: i32) -> i32 {
    let k = k as usize;
    let mut map = [0; 26];

    // We can use the values as bytes since they are English letters only
    let s = s.as_bytes();

    (0..s.len())
        .fold((0, 0, 0), |(mut result, mut max_freq, mut left), right| {
            result += 1;
            let right_key = (s[right] - b'A') as usize;
            map[right_key] += 1;
            max_freq = max_freq.max(map[right_key]);

            if (right - left + 1) - max_freq > k {
                let left_key = (s[left] - b'A') as usize;
                map[left_key] -= 1;
                left += 1;
                result -= 1;
            }

            (result, max_freq, left)
        })
        .0 as i32
}
