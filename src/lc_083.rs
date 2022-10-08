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
    // 耗时24ms，待优化
    fn do_delete(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        // 1. 第一次克隆
        // let mut v = head.clone().as_ref().unwrap().val;
        let mut v = head.as_ref().unwrap().val;
        // 2. 第二次克隆，生成工作链表
        let mut head = head.clone();
        let mut node = &mut head;

        // 遍历所有节点
        // while node.is_some() && node.as_ref().unwrap().next.is_some() {
        while let Some(curr) = node.as_ref() {
            // next节点和当前节点相同
            if curr.next.is_none() {
                break;
            }
            if curr.next.as_ref().unwrap().val == v {
                // 3. 每次相同节点都克隆，从参考链表
                node.as_mut().unwrap().next = curr.next.as_ref().unwrap().next.clone();
            } else {
                // v = curr.next.as_ref().unwrap().val;
                node = &mut node.as_mut().unwrap().next;
                v = node.as_ref().unwrap().val;
            }
        }

        return head.clone();
    }
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 5. clone
        return Solution::do_delete(&mut head.clone());
    }
}
