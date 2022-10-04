struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        return n.count_ones() == 1 && n > 0;
    }
}

#[test]
#[allow(overflowing_literals)]
fn test_is_power_of_two() {
    let mut ans = Solution::is_power_of_two(10);
    assert!(ans == false);
}
