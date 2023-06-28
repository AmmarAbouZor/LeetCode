pub fn main() {
    println!("Search Rotated Sorted Array");
}

// O(log(n)) Binary search
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        // If the left half is the sorted one
        if nums[mid] >= nums[left] {
            if target > nums[mid] || target < nums[left] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        } else {
            if target < nums[mid] || target > nums[right] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }

    -1
}

pub fn search_built_in(nums: Vec<i32>, target: i32) -> i32 {
    let split = nums.partition_point(|&x| x >= nums[0]);

    if target < nums[0] {
        nums[split..]
            .binary_search(&target)
            .map_or(-1, |index| (index + split) as i32)
    } else {
        nums[0..split]
            .binary_search(&target)
            .map_or(-1, |idx| idx as i32)
    }
}
