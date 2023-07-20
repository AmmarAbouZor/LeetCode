pub fn main() {
    println!("Longest Substring Without Repeating Characters");
}

// O(n)
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;

    let mut char_pos_map = std::collections::HashMap::new();

    let mut left = -1;

    for (right, ch) in s.chars().enumerate() {
        let right = right as i32;
        // HashMap return the old value if it already exists
        if let Some(i) = char_pos_map.insert(ch, right) {
            left = left.max(i);
        }

        max = max.max(right - left);
    }

    max
}

// O(n2)
pub fn length_of_longest_substring_slow_short(s: String) -> i32 {
    let mut max = 0;

    for i in 0..s.len() {
        let mut set = std::collections::HashSet::new();

        let count = s.chars().skip(i).take_while(|&ch| set.insert(ch)).count() as i32;

        max = max.max(count);
    }

    max
}

// O(n2)
pub fn length_of_longest_substring_slow(s: String) -> i32 {
    let mut max = 0;

    for (i, ch) in s.chars().enumerate() {
        let mut set = std::collections::HashSet::new();

        let mut count = 1;

        set.insert(ch);

        for ch_next in s.chars().skip(i + 1) {
            if set.insert(ch_next) {
                count += 1;
            } else {
                break;
            }
        }

        max = max.max(count);
    }

    max
}
