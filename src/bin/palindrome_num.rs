pub fn main() {
    println!("Palindrome Nubmer");
}

pub fn is_palindrome(x: i32) -> bool {
    // let num = format!("{}", x);
    let num = x.to_string();

    let iter = num.chars();

    iter.clone().eq(iter.rev())
}

pub fn is_palindrome_math(x: i32) -> bool {
    // revers the number mathematically.
    let mut acc = x;
    let mut reversed = 0;
    while acc > 0 {
        reversed = reversed * 10 + acc % 10;
        acc /= 10;
    }

    // Compare reversed to the given number
    reversed == x
}
