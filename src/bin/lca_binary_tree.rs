pub fn main() {
    println!("Lowest Common Ancestor of a Binary Search Tree");
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
    // O(Logn)
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (&root, &p, &q) {
            (Some(node_r), Some(node_p), Some(node_q)) => {
                let node_r = node_r.borrow();
                let node_p = node_p.borrow();
                let node_q = node_q.borrow();

                match (node_r.val, node_p.val, node_q.val) {
                    // One of the values is the root
                    (rv, pv, qv) if rv == pv || rv == qv => root.clone(),
                    // Nodes are on different branches from the root
                    (rv, pv, qv) if (pv.min(qv)..qv.max(pv)).contains(&rv) => root.clone(),
                    // Nodes are on the same direction from the root
                    (rv, pv, _) => {
                        if rv > pv {
                            // Both on the left
                            Self::lowest_common_ancestor(node_r.left.clone(), p.clone(), q.clone())
                        } else {
                            // Both on the right
                            Self::lowest_common_ancestor(node_r.right.clone(), p.clone(), q.clone())
                        }
                    }
                }
            }
            _ => root,
        }
    }
}
