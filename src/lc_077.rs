struct Solution;

impl Solution {
    fn solve(n: i32, pos: i32, k: i32, temp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if temp.len() > k as usize {
            return;
        }

        if temp.len() == k as usize {
            ans.push(temp.clone());
            return;
        }

        for i in pos..n {
            temp.push(i + 1);
            Solution::solve(n, i + 1, k, temp, ans);
            temp.pop();
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut temp = Vec::new();
        Solution::solve(n, 0, k, &mut temp, &mut ans);
        return ans.into();
    }
}

#[test]
fn test_combine() {
    let n = 4;
    let k = 1;
    let ans = Solution::combine(n, k);
    let mut exp = vec![];
    exp.push(vec![1]);
    exp.push(vec![2]);
    exp.push(vec![3]);
    exp.push(vec![4]);
    assert_eq!(exp, ans);
}
