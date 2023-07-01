pub fn main() {
    println!("Longest Consecutive Sequence");
    longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
}

// O(n * m) use HashSet and most elegant way
pub fn longest_consecutive_one(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.into_iter().collect();

    set.iter()
        .filter(|&x| !set.contains(&(x - 1)))
        .map(|&x| (x..).take_while(|x| set.contains(x)).count())
        .max()
        .unwrap_or(0) as i32
}

// O(n * m) use HashSet
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut max = 0;

    for &num in set.iter() {
        if !set.contains(&(num - 1)) {
            let count = (num..).take_while(|x| set.contains(x)).count();
            max = max.max(count);
        }
    }

    max as i32
}

// O(nlogn)
pub fn longest_consecutive_slow(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    nums.sort();

    dbg!(&nums);

    let mut longest = 1;
    let mut max = 1;

    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        match diff {
            1 => {
                longest += 1;
                max = max.max(longest);
            }
            0 => {}
            _ => longest = 1,
        };
    }

    max
}
