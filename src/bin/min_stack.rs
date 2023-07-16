pub fn main() {
    println!("Min Stack");
}

struct MinStack {
    stack: Vec<Pair>,
}

struct Pair {
    val: i32,
    min: i32,
}

impl Pair {
    fn new(val: i32, min: i32) -> Self {
        Self { val, min }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::with_capacity(30000),
        }
    }

    fn push(&mut self, val: i32) {
        let min = self.stack.last().map_or(i32::MAX, |pair| pair.min).min(val);
        self.stack.push(Pair::new(val, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().min
    }
}
