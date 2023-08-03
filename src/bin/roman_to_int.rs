pub fn main() {
    println!("Roman to Int");
}

// The solution is to read the values backwards and compare the accumulated value
// to determine if the value is negative of positive
// Time: O(n) Space O(1)
pub fn roman_to_int(s: String) -> i32 {
    s.as_bytes().iter().rfold(0, |acc, ch| {
        acc + match ch {
            b'I' => {
                if acc >= 5 {
                    -1
                } else {
                    1
                }
            }
            b'V' => 5,
            b'X' => {
                if acc >= 50 {
                    -10
                } else {
                    10
                }
            }
            b'L' => 50,
            b'C' => {
                if acc >= 500 {
                    -100
                } else {
                    100
                }
            }
            b'D' => 500,
            b'M' => 1000,
            _ => unreachable!(),
        }
    })
}
