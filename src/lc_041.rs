struct Solution;

impl Solution {
    fn solve(data: &mut Vec<i32>) -> i32 {
        let mut idx = data.len() as i32;
        // 让所有元素大于0
        for i in 0..data.len() {
            if data[i] <= 0 {
                data[i] = data.len() as i32 + 1;
            }
        }
        // hashmap
        for i in 0..data.len() {
            let num = i32::abs(data[i]);
            if num - 1 < data.len() as i32 {
                data[num as usize - 1] = -i32::abs(data[num as usize - 1]);
            }
        }

        for i in 0..data.len() {
            if data[i] > 0 {
                idx = i as i32;
                break;
            }
        }
        return idx + 1;
    }
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut data = nums.clone();

        return Solution::solve(&mut data);
    }
}

#[test]
fn test_first_missing_positive() {
    let mut nums = vec![7, 8, 9, 11, 12];
    let mut ans = Solution::first_missing_positive(nums);
    assert_eq!(1, ans);
    nums = vec![3, 4, -1, 1];
    ans = Solution::first_missing_positive(nums);
    assert_eq!(2, ans);
    nums = vec![1, 2, 0];
    ans = Solution::first_missing_positive(nums);
    assert_eq!(3, ans);
    nums = vec![1];
    ans = Solution::first_missing_positive(nums);
    assert_eq!(2, ans);
}
