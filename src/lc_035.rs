/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

use core::num;

// @lc code=start
struct Solution;

impl Solution {
    fn binary_search(nums: &Vec<i32>, mut left: i32, mut right: i32, target: i32) -> i32 {
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return right + 1;
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::binary_search(&nums, 0, nums.len() as i32 - 1, target);
    }
}

#[test]
fn test_search_insert() {
    let mut nums = vec![1, 3, 5, 6];
    let mut target = 5;
    let mut ans = Solution::search_insert(nums.clone(), target);
    assert_eq!(2, ans);
    target = 2;
    ans = Solution::search_insert(nums.clone(), target);
    assert_eq!(1, ans);
}
// @lc code=end
