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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let h1 = list1.as_ref().unwrap().val;
        let h2 = list2.as_ref().unwrap().val;

        if h1 < h2 {
            return Some(Box::new(ListNode {
                val: h1,
                next: Solution::merge_two_lists(list1.unwrap().next, list2),
            }));
        } else {
            return Some(Box::new(ListNode {
                val: h2,
                next: Solution::merge_two_lists(list1, list2.unwrap().next),
            }));
        }
    }
}
