pub fn main() {
    println!("Unique Paths");
    unique_paths(3, 7);
}

// O(mn)
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![vec![0; n + 1]; m + 1];

    dp[m - 1][n - 1] = 1;

    for row in (0..m).rev() {
        for col in (0..n).rev() {
            dp[row][col] += dp[row + 1][col] + dp[row][col + 1];
        }
    }

    dp[0][0]
}
