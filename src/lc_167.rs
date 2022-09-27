use core::num;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left <= right {
            if numbers[left] + numbers[right] < target {
                left += 1;
            } else if numbers[left] + numbers[right] > target {
                right -= 1
            } else {
                return vec![left as i32, right as i32];
            }
        }
        // can not reach
        return vec![];
    }
}
