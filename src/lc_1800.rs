struct Solution;
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut idx: usize = 1;
        let mut curr = nums[0];
        let mut sum = curr;
        let mut max = 0;
        while idx < nums.len() {
            if nums[idx] <= curr {
                max = if sum > max { sum } else { max };
                sum = nums[idx];
            } else {
                sum += nums[idx];
            }
            curr = nums[idx];
            idx += 1;
        }
        return if sum > max { sum } else { max };
    }
}

#[test]
fn test_max_ascending_sum() {
    let mut input = vec![10, 20, 30, 5, 10, 50];
    let mut ans = Solution::max_ascending_sum(input.clone());
    assert_eq!(ans, 65);
    input = vec![10, 20, 30, 40, 50];
    ans = Solution::max_ascending_sum(input.clone());
    assert_eq!(ans, 150);
}
