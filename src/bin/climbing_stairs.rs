pub fn main() {
    println!("Climbing Stairs");
}

// Fibbonacci O(n)
pub fn climb_stairs(n: i32) -> i32 {
    (0..n).fold((1, 0), |(res, prev), _| (res + prev, res)).0
}

// Fibbonacci O(n)
pub fn climb_stairs_simple(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }

    b
}
