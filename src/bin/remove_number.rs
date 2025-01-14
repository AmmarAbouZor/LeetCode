pub fn main() {
    println!("Remove Number");
}

// Time: O(n)
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut idx = 0;
    for j in 0..nums.len() {
        if nums[j] != val {
            nums[idx] = nums[j];
            idx += 1;
        }
    }

    idx as i32
}

// Time: O(m * n), Space: O(1)
pub fn str_str(haystack: String, needle: String) -> i32 {
    for i in 0..haystack.len() {
        if haystack[i..].starts_with(needle.as_str()) {
            return i as i32;
        }
    }

    return -1;
}
