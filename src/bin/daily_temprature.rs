pub fn main() {
    println!("Daily Temprature");
}

// Monotonic Stack Time: O(n) Stack O(n)
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut results = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while stack
            .last()
            .map_or(false, |&j| temperatures[j] < temperatures[i])
        {
            let idx = stack.pop().unwrap();
            results[idx] = (i - idx) as i32
        }

        stack.push(i);
    }

    results
}
