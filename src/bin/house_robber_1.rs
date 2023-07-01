pub fn main() {
    println!("House Robber 1");
}

// DP O(n)
pub fn rob_clean(nums: Vec<i32>) -> i32 {
    let (mut a, mut b, mut best) = (0, 0, 0);
    for num in nums {
        best = b.max(a + num);

        a = b;
        b = best
    }

    best
}

// DP O(n)
pub fn rob_oneline(nums: Vec<i32>) -> i32 {
    let (a, b) = nums.iter().fold((0, 0), |(a, b), num| (b, b.max(a + num)));
    a.max(b)
}

// DP O(n)
pub fn rob_my(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[1];
    }

    let mut dp = vec![0; nums.len()];

    dp[0] = nums[0];
    dp[1] = nums[1];

    for i in 2..nums.len() {
        let potent_current = dp[i - 2] + nums[i];
        let previous = dp[i - 1];
        dp[i] = previous.max(potent_current);
    }

    dp[nums.len() - 1]
}

pub fn rob_var(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[1];
    }

    let mut dp_2 = nums[0];
    let mut dp_1 = nums[1];

    let mut current = 0;

    for i in 2..nums.len() {
        current = dp_1.max(dp_2 + nums[i]);

        dp_2 = dp_1;
        dp_1 = current;
    }

    current
}
