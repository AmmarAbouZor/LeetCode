pub fn main() {
    println!("Reverse Linked List");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);

    while let Some(mut node) = curr {
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}
