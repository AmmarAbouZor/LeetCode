pub fn main() {
    println!("Valid Parentheses");
}

// O(n)
pub fn is_valid(s: String) -> bool {
    let mut queue = std::collections::VecDeque::new();

    for ch in s.chars() {
        if matches!(ch, '(' | '[' | '{') {
            queue.push_front(ch);
        } else if matches!(ch, ')' | ']' | '}') {
            match (queue.pop_front(), ch) {
                (Some('('), ')') => {}
                (Some('['), ']') => {}
                (Some('{'), '}') => {}
                _ => return false,
            };
        }
    }

    queue.is_empty()
}
