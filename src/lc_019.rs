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

struct NodeRef<'a> {
    next: &'a mut ListNode,
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut new_head = head.clone();
        // 设置
        let mut second: Option<&mut ListNode> = None;
        let mut first = head.clone();
        let mut idx = 1;
        while first.is_some() && idx < n {
            first = first.unwrap().next;
            idx += 1;
        }

        // 按照题意，n不会超过链表长度
        // if first.is_none() {
        //     return None;
        // }

        // 找删除节点的上一个节点
        while first.is_some() && first.as_ref().unwrap().next.is_some() {
            if second.is_none() {
                second = Some(new_head.as_deref_mut().unwrap());
            } else {
                second = second.unwrap().next.as_deref_mut();
            }
            // second = second.unwrap().next;
            first = first.unwrap().next;
        }

        // 删除头节点
        if second.is_none() {
            return head.unwrap().next;
        }
        second.unwrap().next = second
            .as_ref()
            .unwrap()
            .next
            .clone()
            .as_ref()
            .unwrap()
            .next
            .clone();
        return new_head;
    }
}

#[test]
fn test_remove_list_node() {
    let mut list = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));
    list.as_mut().unwrap().next = list.clone().unwrap().next.unwrap().next;
    println!("{:?}", list);

    let mut a = Box::new(1);
    let b = a.as_mut();
    *b = 2;

    println!("{:?}", a);
}
