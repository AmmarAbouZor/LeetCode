pub fn main() {
    println!("Tow Sum II, Input Array is Sorted");
}

// Two Pointers O(n)
pub fn two_sum_smart(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_enum = numbers.iter().enumerate();
    let mut left = num_enum.next();
    let mut right = num_enum.next_back();

    while let (Some((idx_l, &num_l)), Some((idx_r, &num_r))) = (left, right) {
        match num_r + num_l {
            num if num == target => return vec![idx_l as i32 + 1, idx_r as i32 + 1],
            num if num < target => left = num_enum.next(),
            _ => right = num_enum.next_back(),
        }
    }

    unreachable!()
}

// My solution O(nlogn)
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &num) in numbers.iter().enumerate() {
        if num * 2 == target {
            if let Some(&next) = numbers.get(i + 1) {
                if num == next {
                    return vec![i as i32 + 1, i as i32 + 2];
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        let remainder = target - num;
        if let Ok(idx) = numbers.binary_search(&remainder) {
            return vec![i as i32 + 1, idx as i32 + 1];
        }
    }

    unreachable!()
}
