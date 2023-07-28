pub fn main() {
    println!("Permutation in Strings");
}

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    // Input is lowercase English letters
    let mut ch1_map = [0; 26];
    let mut ch2_map = [0; 26];

    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    // Fill the initial length of s1
    for i in 0..s1.len() {
        ch1_map[(s1[i] - b'a') as usize] += 1;
        ch2_map[(s2[i] - b'a') as usize] += 1;
    }

    // calculate the initial matches
    let mut matches = (0..26).filter(|&i| ch1_map[i] == ch2_map[i]).count();

    let mut left = 0;
    for right in s1.len()..s2.len() {
        if matches == 26 {
            return true;
        }

        // Calculate the differences in matches with the added character on the right
        let right_key = (s2[right] - b'a') as usize;
        ch2_map[right_key] += 1;
        if ch1_map[right_key] == ch2_map[right_key] {
            matches += 1;
            // It was matched before and now it's not matched anymore
        } else if ch1_map[right_key] + 1 == ch2_map[right_key] {
            matches -= 1;
        }

        // Calculate the differences in matches with the removed character on the left
        let left_key = (s2[left] - b'a') as usize;
        ch2_map[left_key] -= 1;
        if ch1_map[left_key] == ch2_map[left_key] {
            matches += 1;
            // It was matched before and now it's not matched anymore
        } else if ch1_map[left_key] - 1 == ch2_map[left_key] {
            matches -= 1;
        }

        left += 1;
    }

    matches == 26
}
