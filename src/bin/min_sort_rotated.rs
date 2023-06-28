pub fn main() {
    println!("Minimum Rotated Sorted Array");
}

// O(log(n)) binary search
pub fn find_min(nums: Vec<i32>) -> i32 {
    let start = 0;
    let end = nums.len() - 1;

    let middle = end / 2;

    match (nums[start], nums[end]) {
        (start, end) if start == end => return end,
        (start, end) if start > end => find_min(nums[middle..].into_iter().cloned().collect()),
        (start, end) if start < end => find_min(nums[..=middle].into_iter().cloned().collect()),
        _ => unreachable!(),
    }
}

pub fn find_min_no_rec(mut nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;

    while start < end {
        let mid = start + (end - start) / 2;
        if nums[mid] > nums[end] {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    nums[start]
}
