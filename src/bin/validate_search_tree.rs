pub fn main() {
    println!("Validate Binary Search Tree");
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_boundary(root.as_ref(), i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }

    fn is_valid_boundary(root: Option<&Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(ref node) = root {
            let node = node.borrow();

            let val = node.val as i64;

            val > min
                && val < max
                && Self::is_valid_boundary(node.left.as_ref(), min, val)
                && Self::is_valid_boundary(node.right.as_ref(), val, max)
        } else {
            true
        }
    }
}
