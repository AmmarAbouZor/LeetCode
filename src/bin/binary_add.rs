pub fn main() {
    println!("Binary add");
}

// O(1)
pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let carry = a & b; // carry with AND
        a = a ^ b; // add with XOR
        b = carry << 1; // b is the carry shifted to the left
    }

    return a;
}

pub fn get_sum_rec(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_sum(a ^ b, (a & b) << 1)
    }
}
