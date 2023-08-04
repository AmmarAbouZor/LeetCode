pub fn main() {
    println!("Find the Index of the First Occurrence in a String");
}

pub fn str_str_built_in(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map(|idx| idx as i32).unwrap_or(-1)
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    let win_size = needle.len();

    if win_size == 0 {
        return 0;
    }

    haystack
        .windows(win_size)
        .position(|slice| slice == needle)
        .map(|idx| idx as i32)
        .unwrap_or(-1)
}
