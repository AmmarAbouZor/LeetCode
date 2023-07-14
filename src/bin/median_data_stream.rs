pub fn main() {
    println!("Find Median from Data Stream");
}

use std::{cmp::Reverse, collections::BinaryHeap};
#[derive(Default)]
struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
    is_high_turn: bool,
}

// Two heaps must be kept with the same length. Lo then High
impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.is_high_turn {
            self.lo.push(num);
            self.lo.pop().map(|n| self.hi.push(Reverse(n)));
        } else {
            self.hi.push(Reverse(num));
            self.hi.pop().map(|n| self.lo.push(n.0));
        }

        self.is_high_turn = !self.is_high_turn;
    }

    fn find_median(&self) -> f64 {
        // The whole length will be odd when it's high turn
        // Then get the max of Lo
        if self.is_high_turn {
            *self.lo.peek().unwrap() as f64
        } else {
            // Take the max of low and the min of high
            let lo = *self.lo.peek().unwrap();
            let hi = self.hi.peek().unwrap().0;

            (lo + hi) as f64 / 2_f64
        }
    }
}
