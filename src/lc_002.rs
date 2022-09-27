/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

use std::process::id;

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // #[inline]
    // fn new(val: i32) -> Self {
    // 	ListNode {
    // 		next: None,
    // 		val
    // 	}
    // }
}

struct Solution;

// a = a + b
fn sum_of_vector(a: &mut Vec<i32>, b: &Vec<i32>) {
    let mut carry = 0;

    if b.len() > a.len() {
        for _i in 0..b.len() - a.len() {
            a.push(0);
        }
    }

    for (idx, v) in a.into_iter().enumerate() {
        let temp = *v + b.get(idx).map_or(0, |v| *v) + carry;
        *v = temp % 10;
        carry = temp / 10;
    }
    if carry > 0 {
        a.push(carry);
    }
}

fn ll_to_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut current = l;
    let mut result = vec![];

    while current.is_some() {
        result.push(current.clone().unwrap().val);
        current = current.unwrap().next;
    }

    result
}

fn vec_to_ll(v: &mut Vec<i32>) -> Option<Box<ListNode>> {
    let mut result = None;
    v.reverse();
    for item in v {
        let node = ListNode {
            val: *item,
            next: result,
        };
        result = Some(Box::new(node));
    }
    result
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut a = ll_to_vec(l1);
        let b = ll_to_vec(l2);
        sum_of_vector(&mut a, &b);
        vec_to_ll(&mut a)
    }
}
// @lc code=end

#[test]
fn test_add_two_numbers() {
    let l1 = Some(Box::new(ListNode {
        next: None,
        val: 99,
    }));
    Solution::add_two_numbers(l1, None);
}

#[test]
fn test_sum_of_vector() {
    let mut a = vec![9, 9, 9, 9, 9, 9, 9];
    let b = vec![9, 9, 9, 9];
    sum_of_vector(&mut a, &b);
    assert_eq!(vec![8, 9, 9, 9, 0, 0, 0, 1], a);
}
