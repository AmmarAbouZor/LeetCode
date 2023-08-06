pub fn main() {
    println!("Palindrome Nubmer");
}

pub fn is_palindrome(x: i32) -> bool {
    // let num = format!("{}", x);
    let num = x.to_string();

    let iter = num.chars();

    iter.clone().eq(iter.rev())
}
