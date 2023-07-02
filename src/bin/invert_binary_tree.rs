pub fn main() {
    println!("Invert Binary Tree");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // Time: O(n) Space O(n)
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|root| {
            {
                let mut node = root.borrow_mut();
                let left = node.left.clone();
                let right = node.right.clone();
                node.left = Self::invert_tree(right);
                node.right = Self::invert_tree(left);
            }
            root
        })
    }

    // Time: O(n) Space O(n)
    pub fn invert_tree_cleaner(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref n) = root {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            let mut n = n.borrow_mut();

            n.left = Self::invert_tree_cleaner(right);
            n.right = Self::invert_tree_cleaner(left);
        }

        root
    }
}
