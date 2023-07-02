pub fn main() {
    println!("Binary Tree Level Order Traversal");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // O(n): Breadth First Search (BFS)
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut deque = std::collections::VecDeque::new();

        deque.push_back(root.clone());

        while !deque.is_empty() {
            let l = deque.len();
            let mut sub_vec = Vec::with_capacity(l);
            for _ in 0..l {
                let node = deque.pop_front().unwrap();
                if let Some(node) = node {
                    let node = node.borrow();

                    deque.push_back(node.left.clone());
                    deque.push_back(node.right.clone());

                    sub_vec.push(node.val);
                }
            }
            if !sub_vec.is_empty() {
                result.push(sub_vec);
            }
        }

        result
    }

    // O(n) Memory efficient
    pub fn level_order_no_clone(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut deque = std::collections::VecDeque::new();

        deque.push_back(root.clone());

        while !deque.is_empty() {
            let l = deque.len();
            let mut sub_vec = Vec::with_capacity(l);
            for _ in 0..l {
                let node = deque.pop_front().unwrap();
                if let Some(node) = node {
                    deque.push_back(node.borrow_mut().left.take());
                    deque.push_back(node.borrow_mut().right.take());

                    sub_vec.push(node.borrow().val);
                }
            }
            if !sub_vec.is_empty() {
                result.push(sub_vec);
            }
        }

        result
    }
}
