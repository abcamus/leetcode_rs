struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;
        let mut pos = 0;
        while idx < nums.len() {
            if nums[idx] != 0 {
                nums[pos] = nums[idx];
                if idx != pos {
                    nums[idx] = 0
                }
                pos += 1;
            }
            idx += 1;
        }
    }
}
