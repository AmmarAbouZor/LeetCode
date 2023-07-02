pub fn main() {
    println!("Merge Two Sorted Lists");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// O(n2)
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(node1), Some(node2)) => {
            if node1.val > node2.val {
                Some(Box::new(ListNode {
                    val: node2.val,
                    next: merge_two_lists(Some(node1), node2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: node1.val,
                    next: merge_two_lists(node1.next, Some(node2)),
                }))
            }
        }
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (None, None) => None,
    }
}

// O(n) Space O(1)
// Hard to understand. Stick with the recursive
pub fn merge_two_lists_no_rec(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut r = &mut list1;

    while list2.is_some() {
        if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
            std::mem::swap(r, &mut list2);
        }
        r = &mut r.as_mut()?.next;
    }

    list1
}
