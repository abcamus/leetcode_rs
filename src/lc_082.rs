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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut v = head.as_ref().unwrap().val;
        // 原始链表
        let mut walker = &head.clone();
        // 最终链表，卫星节点
        let mut head = ListNode { val: 0, next: None };
        let mut node = &mut head;

        while walker.as_ref().is_some() {
            if walker.as_ref().unwrap().val == v
                && walker.as_ref().unwrap().next.is_some()
                && walker.as_ref().unwrap().next.as_ref().unwrap().val == v
            {
                // 找到下一个非重复节点
                while let Some(next) = walker.as_ref() {
                    if next.val == v {
                        walker = &next.next;
                    } else {
                        break;
                    }
                }
                v = walker
                    .as_ref()
                    .unwrap_or(&Box::new(ListNode {
                        val: 101,
                        next: None,
                    }))
                    .val;
            } else {
                // 将walker添加到node中
                node.next = walker.clone();
                if let Some(n) = node.next.as_mut() {
                    n.next = None;
                }
                // 刷新下一个待更新节点
                node = node.next.as_mut().unwrap();
                // 更新walker
                walker = &walker.as_ref().unwrap().next;
                // 更新value
                v = walker
                    .as_ref()
                    .unwrap_or(&Box::new(ListNode {
                        val: 101,
                        next: None,
                    }))
                    .val;
            }
        }

        return head.next;
    }
}
