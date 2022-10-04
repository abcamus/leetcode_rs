struct Solution;

impl Solution {
    fn solve(n: u32) -> i32 {
        let mut n = n;
        let mut ans = 0;
        while n != 0 {
            n &= n - 1;
            ans += 1;
        }
        return ans;
    }
    pub fn hammingWeight(n: u32) -> i32 {
        // return n.count_ones() as i32;
        return Solution::solve(n);
    }
}
