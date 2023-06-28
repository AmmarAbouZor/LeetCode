pub fn main() {
    println!("Container With Most Water");
}

// Basic O(n2)
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut area = i32::MIN;

    for i in 0..height.len() {
        for j in 0..height.len() {
            if i == j {
                continue;
            }

            let current = i32::abs(i as i32 - j as i32) * height[i].min(height[j]);
            area = area.max(current);
        }
    }

    area
}

// O(n) using Iteration from both sides
pub fn max_area_double_iter(height: Vec<i32>) -> i32 {
    let mut area = i32::MIN;

    let mut iter_enum = height.into_iter().enumerate();

    let mut left = iter_enum.next();
    let mut right = iter_enum.next_back();

    while let (Some((li, lh)), Some((ri, rh))) = (left, right) {
        let current = lh.min(rh) * (ri - li) as i32;
        area = area.max(current);

        if lh < rh {
            left = iter_enum.next();
        } else {
            right = iter_enum.next_back();
        }
    }

    area
}
