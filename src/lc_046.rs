use std::collections::HashSet;

struct Solution;
impl Solution {
    fn solve(
        nums: &Vec<i32>,
        record: &mut HashSet<i32>,
        temp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if temp.len() == nums.len() {
            ans.push(temp.clone());
        }

        for i in 0..nums.len() {
            if record.contains(&nums[i]) {
                continue;
            }
            record.insert(nums[i]);
            temp.push(nums[i]);
            Solution::solve(nums, record, temp, ans);
            temp.pop();
            record.remove(&nums[i]);
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut temp = Vec::new();
        let mut record = HashSet::new();
        Solution::solve(&nums, &mut record, &mut temp, &mut ans);
        return ans.into();
    }
}

#[test]
fn test_permute() {
    let input = vec![1, 2, 3, 4];
    let ans = Solution::permute(input);
    // println!("{:?}", ans);
    assert_eq!(ans.len(), 24);
}
