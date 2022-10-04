use std::cmp::max;

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut nums = vec![0; 46];
        nums[1] = 1;
        nums[2] = 2;
        for i in 3..(n + 1) as usize {
            nums[i] = nums[i - 1] + nums[i - 2];
        }

        return nums[n as usize];
    }
}

#[test]
fn test_climb_stairs() {
    let ans = Solution::climb_stairs(10);
    assert_eq!(ans, 89);
}
