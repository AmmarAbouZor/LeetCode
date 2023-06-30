pub fn main() {
    println!("Combination Sum IV");

    combination_sum4(vec![1, 2, 3], 4);
}

// DP bottom up O(nm)
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let mut dp = vec![0; target + 1];
    dp[0] = 1;

    for i in 1..=target as usize {
        for &n in nums.iter() {
            if i >= n as usize {
                dp[i] += dp[i - n as usize];
            }
        }
    }

    dp[target]
}
