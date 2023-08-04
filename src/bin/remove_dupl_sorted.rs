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
