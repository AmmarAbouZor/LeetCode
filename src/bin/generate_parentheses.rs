pub fn main() {
    println!("Generate Parentheses");

    let _ = generate_parenthesis(3);
}

// My Solution
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut result = Vec::new();

    let mut stack = Vec::with_capacity(n + 1);

    stack.push((String::from("("), 1));

    while let Some(mut pair) = stack.pop() {
        if pair.1 > n {
            continue;
        }

        if pair.1 == n {
            let mut comb = pair.0;
            while comb.len() < n * 2 {
                comb.push(')');
            }
            result.push(comb);

            continue;
        }

        if pair.0.len() < pair.1 * 2 {
            let mut clone = pair.clone();
            clone.0.push(')');
            stack.push(clone);
        }

        pair.0.push('(');
        pair.1 += 1;
        stack.push(pair);
    }

    result
}

pub fn generate_parenthesis_backtracing(n: i32) -> Vec<String> {
    fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
        let mut res = Vec::new();

        if open == 0 && close == 0 {
            return vec![s];
        }

        if open.is_positive() {
            let mut bck_track = back_track(s.clone() + "(", open - 1, close + 1);
            res.append(&mut bck_track);
        }
        if close.is_positive() {
            let mut bck_track = back_track(s.clone() + ")", open, close - 1);
            res.append(&mut bck_track);
        }

        res
    }

    back_track(String::new(), n, 0)
}
