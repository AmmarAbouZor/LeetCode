pub fn main() {
    println!("Maximume Product Subarray");
}

// O(n) Kadane's Algo
pub fn max_product(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold((i32::MIN, 1), |(max, curr), num| {
            let current = num.max(num * curr);
            (max.max(current), current)
        })
        .0
}
