pub fn main() {
    println!("Product except self");
}

// O(n): one loop with two indices
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prod = vec![1; nums.len()];
    let mut left_index = 0;
    let mut right_index = nums.len() - 1;

    let mut left_prod = 1;
    let mut right_prod = 1;

    loop {
        prod[left_index] *= left_prod;
        prod[right_index] *= right_prod;

        right_prod *= nums[right_index];
        left_prod *= nums[left_index];

        if right_index == 0 {
            break;
        }

        left_index += 1;
        right_index -= 1;
    }

    prod
}
