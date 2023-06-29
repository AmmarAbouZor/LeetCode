pub fn main() {
    println!("Revert Bits");
}

pub fn reverse_bits(x: u32) -> u32 {
    x.reverse_bits()
}

pub fn reverse_bits_manul(x: u32) -> u32 {
    let (mut res, mut x) = (0u32, x);

    for _ in 0..32 {
        res = (res << 1) | (x & 1);
        x = x >> 1;
    }

    res
}
