pub fn main() {
    println!("Remove Duplicated Soreted Array");
}

pub fn remove_duplicates_built_in(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut prev = 0;
    for i in 1..nums.len() {
        if nums[prev] != nums[i] {
            prev += 1;
            nums[prev] = nums[i];
        }
    }

    (prev + 1) as i32
}

// O(n)
pub fn remove_duplicates_final(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    // First number is always unique.
    let mut unique_idx = 1;

    for right in 1..nums.len() {
        // if the two numbers in sequent are different then add the number to the list in the latest index for unique numbers
        if nums[right] != nums[right - 1] {
            nums[unique_idx] = nums[right];
            unique_idx += 1;
        }
    }

    // // This should be called actually to ensure that the vector is resized.
    // nums.truncate(unique_idx);
    // // In this case we can return the length of the vector directly.
    // nums.len() as i32

    unique_idx as i32
}
