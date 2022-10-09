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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ref_node = &head;
        let mut head = None;
        let mut node: &mut Option<Box<ListNode>> = &mut head;

        while ref_node.is_some() {
            if ref_node.as_ref().unwrap().val != val {
                let temp = ref_node.as_ref().unwrap();
                node.replace(Box::new(ListNode::new(temp.val)));
                node = &mut node.as_mut().unwrap().next;
            }

            ref_node = &ref_node.as_ref().unwrap().next;
        }

        return head;
    }
}
