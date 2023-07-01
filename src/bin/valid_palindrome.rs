pub fn main() {
    println!("Valid Palindrome");
}

// Fast + No allocation
pub fn is_palindrome(s: String) -> bool {
    let iter = s
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase());

    iter.clone().eq(iter.rev())
}

// Fast but more memory
pub fn is_palindrome_collect(s: String) -> bool {
    let s: Vec<char> = s
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    if s.is_empty() {
        return true;
    }

    let mut left = 0;
    let mut right = s.len() - 1;

    while right > left {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
