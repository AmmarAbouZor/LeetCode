pub fn main() {
    println!("Jumb Game");
}

// O(n)
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut goal_idx = nums.len() - 1;

    for i in (0..nums.len()).rev() {
        if i + nums[i] as usize >= goal_idx {
            goal_idx = i;
        }
    }

    goal_idx == 0
}

// O(n) This is faster
pub fn can_jump_2(nums: Vec<i32>) -> bool {
    let mut max_val: usize = 0;

    for (idx, &val) in nums.iter().enumerate() {
        if idx > max_val {
            return false;
        }

        max_val = max_val.max(idx + val as usize);
    }

    true
}
