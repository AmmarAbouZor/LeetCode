pub fn main() {
    println!("Missing Num");
}

// O(n2)
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut num = 0;

    loop {
        if !nums.contains(&num) {
            return num;
        }

        num += 1;
    }
}

// O(n)
pub fn missing_number_diff_sum(nums: Vec<i32>) -> i32 {
    let range_sum = nums.len() * (nums.len() + 1) / 2;
    range_sum as i32 - nums.into_iter().sum::<i32>()
}

// O(n)
pub fn missing_number_xor(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .fold(0, |xor, (i, x)| xor ^ i as i32 + 1 ^ x)
}
