struct Solution;

impl Solution {
    fn binary_search(nums: &Vec<i32>, left: i32, right: i32, target: i32) -> i32 {
        if left > right {
            return -1;
        }
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] > target {
            return Solution::binary_search(nums, left, mid - 1, target);
        } else {
            return Solution::binary_search(nums, mid + 1, right, target);
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::binary_search(&nums, 0, (nums.len() - 1) as i32, target);
    }
}

#[test]
fn test_search() {
    let mut ans = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
    assert_eq!(4, ans);
    let mut ans = Solution::search(vec![-1, 0, 3, 5, 9, 12], 2);
    assert_eq!(-1, ans);
}
