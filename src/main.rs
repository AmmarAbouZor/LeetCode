fn main() {
    println!("Hello, world!");
}

pub fn roman_to_int(s: String) -> i32 {
    s.chars().fold(0, |acc, ch| match ch {
        'I' => acc + 1,
        'V' => acc + 5,
        'X' => acc + 10,
        'L' => acc + 50,
        'C' => acc + 100,
        'D' => acc + 500,
        'M' => acc + 1000,
        _ => unreachable!("Invalid input"),
    })
}
