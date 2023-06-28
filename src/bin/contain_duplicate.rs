use std::collections::HashSet;

pub fn main() {
    println!("Contain Duplicate");
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    !nums.into_iter().all(|n| set.insert(n))
}
