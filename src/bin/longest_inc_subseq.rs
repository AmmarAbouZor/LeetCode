pub fn main() {
    println!("Longest Increasing Subsequence");
}

// Patience Sort O(NlogN)
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut piles = Vec::new();

    for num in nums {
        // binary_search returns Err if number isn't found with the potential  inserting position if
        // for the element to keep the list in the right order
        if let Err(i) = piles.binary_search(&num) {
            if i < piles.len() {
                piles[i] = num;
            } else {
                piles.push(num);
            }
        }
    }

    piles.len() as i32
}

// Patience Sort O(NlogN)
pub fn length_of_lis_one_line(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold(Vec::new(), |mut piles, n| {
            if let Err(i) = piles.binary_search(&n) {
                if i < piles.len() {
                    piles[i] = n;
                } else {
                    piles.push(n);
                }
            }
            piles
        })
        .len() as i32
}

// DP O(n2)
// Go Backward and compare each value with the values in front of it
pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
    let mut lenghts = vec![1; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        for j in i + 1..nums.len() {
            if nums[i] < nums[j] {
                lenghts[i] = lenghts[i].max(1 + lenghts[j]);
            }
        }
    }

    lenghts.into_iter().max().unwrap()
}
