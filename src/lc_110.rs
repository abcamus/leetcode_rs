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
use std::ops::Add;
use std::rc::Rc;

struct Solution;

impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        h_max: &mut i32,
        h_min: &mut i32,
        h: &mut i32,
        valid: &mut bool,
    ) {
        if *valid == false {
            return;
        }
        if root.is_some() {
            *h = h.add(1);
            Solution::dfs(
                root.as_ref().unwrap().borrow().left.clone(),
                h_max,
                h_min,
                h,
                valid,
            );
            *h = h.add(-1);
            Solution::dfs(
                root.as_ref().unwrap().borrow().right.clone(),
                h_max,
                h_min,
                h,
                valid,
            );
            *h = h.add(-1);
        } else {
            // 叶子节点
            if h_max.lt(&h) {
                *h_max = *h;
            }
            *h_max = if h_max.lt(&h) { *h } else { *h_max };
            *h_min = if h_min.gt(&h) { *h } else { *h_min };
            let abs_h = *h_max - *h_min;
            *valid = if abs_h.abs() > 1 { false } else { true }
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_some() {
            let mut h_max = i32::MIN;
            let mut h_min = i32::MAX;
            let mut h = 0;
            let mut valid = true;
            Solution::dfs(root, &mut h_max, &mut h_min, &mut h, &mut valid);
        }
        return true;
    }
}
