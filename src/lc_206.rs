#![allow(unused)]
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();
        let mut node = head;
        while node.is_some() {
            stack.push(Some(Box::new(ListNode::new(node.as_ref().unwrap().val))));
            node = node.unwrap().next;
        }

        if stack.is_empty() {
            return None;
        }

        let mut head = stack.pop().unwrap();
        let mut node = &mut head;

        while !stack.is_empty() {
            node.as_mut().unwrap().next = stack.pop().unwrap();
            node = &mut node.as_mut().unwrap().next;
        }

        node.as_mut().unwrap().next = None;

        return head;
    }
}

#[test]
fn test_box_clone() {
    let mut a = Box::new(1);
    let b = &mut a;
    let c = &mut *b;
    **c = 2;
    *a.as_mut() = 5;
    assert_eq!(*a, 5);
    // let d = &mut *c;
    // *d = 3;
    // assert_eq!(*a, 3);
    // let mut v = Vec::new();
    // for i in 0..5 {
    //     v.push(i);
    // }
}
