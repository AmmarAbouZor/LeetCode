pub fn main() {
    println!("Binary Sreach");
}

// Manually Implemented O(logn)
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = (left + right) / 2;

        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }

    -1
}

pub fn search_built_in(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).map_or(-1, |n| n as i32)
}
