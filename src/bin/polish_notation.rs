pub fn main() {
    println!("Evaluate Reverse Polish Notation");
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();

    for token in tokens.iter() {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                let num_2 = stack.pop().unwrap();
                let num_1 = stack.pop().unwrap();

                let num = match token.as_str() {
                    "+" => num_1 + num_2,
                    "-" => num_1 - num_2,
                    "*" => num_1 * num_2,
                    "/" => num_1 / num_2,
                    _ => unreachable!(),
                };

                stack.push(num);
            }
            num => {
                let num = num.parse().unwrap();
                stack.push(num);
            }
        }
    }

    stack.pop().unwrap()
}
