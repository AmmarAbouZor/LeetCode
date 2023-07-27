pub fn main() {
    println!("Subsets");
}

// Backtracking Time: O(2^n) Space: O(2^n)
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn back_track(sets: &mut Vec<Vec<i32>>, nums: &[i32], mut index: usize) {
        let mut local_sets: Vec<_> = sets
            .iter()
            .cloned()
            .map(|mut v| {
                v.push(nums[index]);
                v
            })
            .collect();

        sets.append(&mut local_sets);

        index += 1;
        if index < nums.len() {
            back_track(sets, nums, index);
        }
    }

    let mut result = vec![Vec::new()];

    back_track(&mut result, &nums, 0);

    result
}
