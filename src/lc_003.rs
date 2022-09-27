/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut left = 0 as usize;
        let mut right = 0 as usize;
        let mut curr_len = 0;
        let mut max_len = curr_len;
        let data = s.as_bytes().to_vec();

        while right < data.len() {
            while left < right && set.contains(&data[right]) {
                set.remove(&data[left]);
                left += 1;
            }
            set.insert(&data[right]);
            curr_len = right - left + 1;
            right += 1;
            max_len = if curr_len > max_len {
                curr_len
            } else {
                max_len
            };
        }

        return max_len as i32;
    }
}
// @lc code=end
