pub fn main() {
    println!(" Kth Smallest Element in a BST");
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
    // Time: O(logn) Space: O(logn)
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut nodes_stack = Vec::new();

        let mut node = root;

        while node.is_some() || !nodes_stack.is_empty() {
            // Go to the smallest node and fill the stack with the nodes in the way
            while let Some(n) = node {
                nodes_stack.push(n.clone());
                node = n.borrow().left.clone();
            }

            // Walk your way up in the stack and check the takes the right nodes with
            if let Some(n) = nodes_stack.pop() {
                k -= 1;

                if k == 0 {
                    return n.borrow().val;
                }

                // Check the right side and take it
                node = n.borrow().right.clone();
            }
        }

        unreachable!()
    }
}
