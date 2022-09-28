// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }
        if root2.is_none() {
            return root1;
        }

        // let a = root1.as_ref().unwrap().borrow().left.clone();

        let new_val = root1.as_ref().unwrap().borrow().val + root2.as_ref().unwrap().borrow().val;
        let left_tree = Solution::merge_trees(
            root1.as_ref().unwrap().borrow().left.clone(),
            root2.as_ref().unwrap().borrow().left.clone(),
        );
        let right_tree = Solution::merge_trees(
            root1.as_ref().unwrap().borrow().right.clone(),
            root2.as_ref().unwrap().borrow().right.clone(),
        );
        return Some(Rc::new(RefCell::new(TreeNode {
            val: new_val,
            left: left_tree,
            right: right_tree,
        })));
    }
}
