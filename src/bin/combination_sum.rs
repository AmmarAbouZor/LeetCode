pub fn main() {
    println!("Combination Sum");
}

// O(2^target)
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    backtrack(Vec::new(), &candidates, target, &mut result);

    result
}

fn backtrack(sub: Vec<i32>, candidates: &[i32], target: i32, res: &mut Vec<Vec<i32>>) {
    let sum: i32 = sub.iter().sum();

    if sum == target {
        res.push(sub);
        return;
    } else if sum > target {
        return;
    }

    for (i, &v) in candidates.iter().enumerate() {
        let mut sub_clone = sub.clone();
        sub_clone.push(v);
        backtrack(sub_clone, &candidates[i..], target, res);
    }
}
