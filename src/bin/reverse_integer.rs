pub fn main() {
    println!("Reverse Integer");
}

pub fn reverse(x: i32) -> i32 {
    reverse_safe(x).unwrap_or(0)
}

fn reverse_safe(x: i32) -> Option<i32> {
    let mut res = 0_i32;
    let mut rem = x;

    const DEC_BASE: i32 = 10;
    while rem != 0 {
        let digit = rem % DEC_BASE;
        rem = rem / DEC_BASE;

        res = res.checked_mul(DEC_BASE)?;
        res = res.checked_add(digit)?;
    }

    Some(res)
}
