pub fn main() {
    println!("Number of 1 Bits");
}

// O(1)
pub fn hamming_weight(n: u32) -> i32 {
    n.count_ones() as i32
}

// O(1)
pub fn hamming_weight_man(n: u32) -> i32 {
    let mut res: i32 = 0;

    for i in 0..32 {
        let mask = 0b1 << i;
        if n & mask != 0 {
            res += 1;
        }
    }

    res
}
