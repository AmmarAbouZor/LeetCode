pub fn main() {
    println!("Single Number");
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| acc ^ n)
}

pub fn single_number_reduce(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|acc, n| acc ^ n).unwrap_or(0)
}
