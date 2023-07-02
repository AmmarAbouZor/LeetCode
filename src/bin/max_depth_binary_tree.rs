pub fn main() {
    println!("Maximum Depth of Binary Tree");
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
    // Time: O(n), Space: O(Logn) Or O(h) depending on the hight
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut left_depth = 0;
        let mut right_depth = 0;

        if let Some(ref node) = root {
            left_depth = Self::max_depth(node.borrow().left.clone()) + 1;
            right_depth = Self::max_depth(node.borrow().right.clone()) + 1;
        }

        left_depth.max(right_depth)
    }
}
