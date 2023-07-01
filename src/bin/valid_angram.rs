pub fn main() {
    println!("Valid Angram");
}

// Time O(n) Space o(n)
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    use std::collections::HashMap;
    let mut count = HashMap::new();

    for ch in s.chars() {
        count.entry(ch).and_modify(|occ| *occ += 1).or_insert(1);
    }

    for ch in t.chars() {
        let contained = count.get(&ch).map_or(false, |&occ| occ > 0);
        if !contained {
            return false;
        }

        count.entry(ch).and_modify(|occ| *occ -= 1);
    }

    true
}

// Faster: Time O(n) Space o(n)
pub fn is_anagram_cleaner(s: String, t: String) -> bool {
    let mut map = std::collections::HashMap::new();

    s.chars().for_each(|c| {
        map.entry(c).and_modify(|c| *c += 1).or_insert(1);
    });

    t.chars().for_each(|c| {
        map.entry(c).and_modify(|c| *c -= 1).or_insert(1);
    });

    map.into_values().all(|count| count == 0)
}
