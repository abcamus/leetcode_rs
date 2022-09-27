use core::num;

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut s = nums[0] * nums[0];
        let mut idx = 1;
        while idx < nums.len() {
            let temp = nums[idx] * nums[idx];
            if temp > s {
                break;
            } else {
                s = temp;
            }
            idx += 1;
        }

        let mut left = idx as i32 - 1;
        let mut right = idx;

        while left >= 0 || right < nums.len() {
            if left < 0 {
                res.push(nums[right] * nums[right]);
                right += 1;
            } else {
                if right >= nums.len() {
                    res.push(nums[left as usize] * nums[left as usize]);
                    left -= 1;
                } else {
                    let a = nums[left as usize] * nums[left as usize];
                    let b = nums[right] * nums[right];
                    if a < b {
                        res.push(a);
                        left -= 1;
                    } else {
                        res.push(b);
                        right += 1;
                    }
                }
            }
        }

        return res;
    }
}
