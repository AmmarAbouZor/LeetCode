pub fn main() {
    println!("Three Sum");
}

// Bad O(n3)
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer = Vec::new();

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            for k in 0..nums.len() {
                if i == j || i == k || j == k {
                    continue;
                }

                if nums[i] + nums[j] + nums[k] == 0 {
                    if answer.iter().all(|v: &Vec<i32>| {
                        !v.contains(&nums[i]) || !v.contains(&nums[j]) || !v.contains(&nums[k])
                    }) {
                        answer.push(vec![nums[i], nums[j], nums[k]]);
                    }
                }
            }
        }
    }

    answer
}

// Best O(n2)
// Sort the list and then try to move the pointers from each end until we get the zero
// The optimizations can be avoided by using a hashmap
pub fn three_sum_sort(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer = Vec::new();

    nums.sort();

    for i in 0..nums.len() {
        // This used to remove the duplications without having to use hashmap
        // We can delete this and use a hashmap instead
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            match sum {
                s if s > 0 => right -= 1,
                s if s < 0 => left += 1,
                0 => {
                    answer.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    // This is optimization only and can be removed
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    answer
}
