pub fn main() {
    println!("Non Overlapping Intervals");
}

// Time O(nlogn) Space O(1)
pub fn erase_overlap_intervals_cleaner(mut intervals: Vec<Vec<i32>>) -> i32 {
    // We can solve this with sorting the intervals ends only
    intervals.sort_by_key(|itrv| itrv[1]);

    let mut count = 0;

    let mut right = intervals[0][1];

    for itvr in intervals.into_iter().skip(1) {
        if itvr[0] < right {
            count += 1;
            // we don't need to make right = min(right, other end) because the vec is already
            // sorted by the end values
        } else {
            right = itvr[1];
        }
    }

    count
}

pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    intervals.sort_by(|a, b| match a[0].cmp(&b[0]) {
        std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
        x => x,
    });

    let mut right = intervals[0][1];

    for i in 1..intervals.len() {
        if intervals[i][0] < right {
            count += 1;

            right = right.min(intervals[i][1]);
        } else {
            right = intervals[i][1];
        }
    }

    count
}
