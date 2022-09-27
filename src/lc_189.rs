use core::num;

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let round = k as usize % len;
        nums.reverse();
        let mut part = &mut nums[0..round];
        part.reverse();
        part = &mut nums[round..len];
        part.reverse();
    }
}
