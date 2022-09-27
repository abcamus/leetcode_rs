use std::cmp::min;

struct Solution;

impl Solution {
    fn solve(height: &Vec<i32>) -> i32 {
        // 单调栈
        let mut stack = Vec::new();
        let mut sum = 0;

        for (idx, h) in height.into_iter().enumerate() {
            while !stack.is_empty() {
                let mut top_idx = *stack.last().unwrap();
                // 当前高度小于最小高度，不能蓄水
                if &height[top_idx] > h {
                    break;
                }

                stack.pop();
                let low = height[top_idx];
                if stack.is_empty() {
                    break;
                }
                // 计算当前top的蓄水量
                top_idx = *stack.last().unwrap();
                let left = height[top_idx];
                let distance = (idx - stack.last().unwrap() - 1) as i32;
                let height = min(*h, left) - low;
                sum += distance * height;
            }
            stack.push(idx);
        }
        return sum;
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        return Solution::solve(&height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trap() {
        let mut height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let mut ans = Solution::trap(height);
        assert_eq!(6, ans);
    }
}
