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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            return true;
        }

        if let Some(node) = root {
            let node = node.borrow();

            Self::is_subtree(node.left.clone(), sub_root.clone())
                || Self::is_subtree(node.right.clone(), sub_root.clone())
        } else {
            false
        }
    }
}
