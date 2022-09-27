use std::{collections::HashMap, slice::SliceIndex};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut map = HashMap::<i32, i32>::new();

        for (idx, v) in nums.into_iter().enumerate() {
            map.get(&(&target - v)).map(|v| {
                ans.push(*v);
                ans.push(idx as i32)
            });
            map.insert(v, idx as i32);
        }
        ans
    }
}

#[test]
fn test_two_sum() {
    let input = vec![1, 2];
    let target = 3;
    assert_eq!(Solution::two_sum(input, target), vec![0, 1]);
}
