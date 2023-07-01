pub fn main() {
    println!("House Robber 2");
}

// O(n)
// Call the rob function from house robber 1 twice for two slices:
// All except the first one and all except the last one
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let candidate_1 = rob_range(&nums[1..]);

    let candidate_2 = rob_range(&nums[..nums.len() - 1]);

    candidate_2.max(candidate_1)
}

fn rob_range(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let (a, b) = nums.iter().fold((0, 0), |(a, b), num| (b, b.max(a + num)));

    a.max(b)
}
