pub fn main() {
    println!("Maximum Subarray");
}

// O(n)
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold((i32::MIN, 0), |(max_sum, sum), num| {
            let current = num.max(sum + num);
            (max_sum.max(current), current)
        })
        .0
}

// O(n): same as above but it's simpler to read
pub fn max_sub_array_simple(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let mut sum = 0;
    for n in nums {
        sum += n;
        max = max.max(sum);
        sum = sum.max(0);
    }
    max
}
