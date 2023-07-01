pub fn main() {
    println!("Top K Frequent Elements");
}

// O(NlogN)
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for num in nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut map: Vec<(i32, usize)> = map.into_iter().collect();

    map.sort_by(|(_, count_a), (_, count_b)| count_b.partial_cmp(count_a).unwrap());

    map.into_iter()
        .take(k as usize)
        .map(|(num, _)| num)
        .collect()
}

// O(N) Binary Heap is very useful for the ordering quests
pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    let k = k as usize;

    for num in nums {
        map.entry(num).and_modify(|count| *count -= 1).or_insert(-1);
    }

    let mut heap = std::collections::BinaryHeap::with_capacity(k);

    map.into_iter().for_each(|(num, count)| {
        heap.push((count, num));

        if heap.len() > k {
            heap.pop();
        }
    });

    heap.into_iter().map(|(_, num)| num).collect()
}
