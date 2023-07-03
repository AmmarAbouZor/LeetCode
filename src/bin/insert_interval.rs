pub fn main() {
    println!("Insert Interval");
}

// Time On(n) Space O(1)
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut less = Vec::new();
    let mut more = Vec::new();

    let mut start = new_interval[0];
    let mut end = new_interval[1];

    for curr in intervals {
        if curr[1] < new_interval[0] {
            less.push(curr);
        } else if curr[0] > new_interval[1] {
            more.push(curr)
        } else {
            start = start.min(curr[0]);
            end = end.max(curr[1]);
        }
    }

    less.push(vec![start, end]);
    less.append(&mut more);

    less
}

pub fn insert_heap(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();

    let mut new_interval = new_interval;

    for interval in intervals.iter() {
        if interval[1] < new_interval[0] {
            ans.push(interval.clone());
        } else if interval[0] > new_interval[1] {
            ans.push(new_interval.clone());
            new_interval = interval.clone();
        } else if interval[1] >= new_interval[0] || interval[0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(interval[0]);
            new_interval[1] = new_interval[1].max(interval[1]);
        }
    }

    ans.push(new_interval.clone());
    ans
}
