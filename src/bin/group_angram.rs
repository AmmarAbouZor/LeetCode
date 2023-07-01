pub fn main() {
    println!("Group Anagram");
}

// O(n2)
pub fn group_anagrams_simple(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();

    for s in strs {
        let mut key: Vec<char> = s.chars().collect();

        key.sort();

        map.entry(key).or_insert(vec![]).push(s);
    }

    map.values().cloned().collect()
}

// O(n2)
// The only difference between the methods is the key in the HashMap
pub fn group_anagrams_efficient(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();

    let offset = b'a' as usize;

    for s in strs {
        let mut key = [0_u8; 26];

        for ch in s.chars() {
            key[ch.to_ascii_lowercase() as usize - offset] += 1;
        }

        map.entry(key).or_insert(vec![]).push(s);
    }

    map.values().cloned().collect()
}
