use std::borrow::Borrow;

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_ptr = head.clone();
        let mut fast_ptr = head.clone();

        while fast_ptr.is_some() && fast_ptr.as_ref().unwrap().next.is_some() {
            slow_ptr = slow_ptr.unwrap().next;
            fast_ptr = fast_ptr.unwrap().next.unwrap().next;
        }

        return slow_ptr;
    }
}
