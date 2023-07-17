pub fn main() {
    println!("Construct Binary Tree");
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index_map = inorder
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();

        Self::build_recursive(
            &mut preorder.iter(),
            &index_map,
            (0, preorder.len() as isize - 1),
        )
    }

    fn build_recursive(
        preorder: &mut std::slice::Iter<i32>,
        index_map: &HashMap<i32, usize>,
        range: (isize, isize),
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if range.0 <= range.1 {
            if let Some(&val) = preorder.next() {
                if let Some(&i) = index_map.get(&val) {
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val: val,
                        left: Self::build_recursive(preorder, index_map, (range.0, i as isize - 1)),
                        right: Self::build_recursive(
                            preorder,
                            index_map,
                            (i as isize + 1, range.1),
                        ),
                    })));
                }
            }
        }

        None
    }
}
