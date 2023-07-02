pub fn main() {
    println!("Same Tree");
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
    // Easiest solution since TreeNode derives Eq
    pub fn is_same_tree_simple(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }

    // O(n) Time: O(logn)
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p_node), Some(q_node)) => {
                p_node.borrow().val == q_node.borrow().val
                    && Self::is_same_tree(
                        p_node.borrow().left.clone(),
                        q_node.borrow().left.clone(),
                    )
                    && Self::is_same_tree(
                        p_node.borrow().right.clone(),
                        q_node.borrow().right.clone(),
                    )
            }
            (None, None) => true,
            _ => false,
        }
    }
}
