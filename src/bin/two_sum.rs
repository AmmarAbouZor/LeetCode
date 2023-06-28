use std::collections::HashMap;

pub fn main() {
    println!("two sum");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    unreachable!();
}

pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diff_hash = HashMap::with_capacity(nums.len());

    for (i, num) in nums.into_iter().enumerate() {
        match diff_hash.get(&num) {
            Some(&j) => return vec![i as i32, j as i32],
            // If it doesn't, add the difference between target and the current number to the hash map
            None => {
                diff_hash.insert(target - num, i);
            }
        }
    }

    unreachable!()
}
