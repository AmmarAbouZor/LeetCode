pub fn main() {
    println!("Count Bits");
}

// O(n * size of i32)
pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n as usize).map(|x| x.count_ones() as i32).collect()
}

// O(n)
pub fn count_bits_odd(n: i32) -> Vec<i32> {
    let n = (n + 1) as usize;

    let mut count = vec![0; n];

    for i in 1..n {
        count[i] = if i % 2 == 0 {
            count[i / 2]
        } else {
            count[i - 1] + 1
        };
    }

    count
}

// O(n)
pub fn count_bits_odd_bit(n: i32) -> Vec<i32> {
    let n = (n + 1) as usize;

    let mut count = vec![0; n];

    for i in 1..n {
        count[i] = if i & 2 == 0 {
            count[i >> 1]
        } else {
            count[i - 1] + 1
        };
    }

    count
}
