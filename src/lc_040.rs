use core::cmp::min;
use std::{collections::HashMap, hash::Hash};

struct Solution;

impl Solution {
    /*
     * 每一步返回包含当前idx位置元素的序列结果
     */
    fn solve(
        candidates: &Vec<i32>,
        idx: usize,
        target: i32,
        count_map: &HashMap<i32, usize>,
        seq: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            println!("found: {:?}", seq);
            result.push(seq.clone());
            return;
        } else {
            if idx >= candidates.len() {
                return;
            }
            for i in idx..candidates.len() {
                if candidates[i] <= target {
                    let max = min(
                        (target / candidates[i]) as usize,
                        *count_map.get(&candidates[i]).unwrap(),
                    );
                    for j in 1..max + 1 {
                        seq.push(candidates[i]);
                        let new_target = target - candidates[i] * j as i32;
                        Solution::solve(candidates, i + 1, new_target, count_map, seq, result);
                    }
                    for _ in 1..max + 1 {
                        seq.pop();
                    }
                }
            }
        }
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // let mut used = HashSet::new();
        let mut ans = Vec::new();
        let mut seq = Vec::new();
        let mut data = Vec::new();
        let mut count_map = HashMap::new();
        for k in &candidates {
            let count = match count_map.get(k) {
                Some(v) => *v,
                None => {
                    data.push(*k);
                    0
                }
            };
            // let count = count_map.get(k).unwrap_or(&(0 as usize));
            count_map.insert(*k, count + 1);
        }
        data.sort();
        println!("{:?}", data);
        Solution::solve(&data, 0 as usize, target, &count_map, &mut seq, &mut ans);
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combination_sum2() {
        let mut candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let mut target = 8;
        let mut ans = Solution::combination_sum2(candidates, target);
        println!("ans: {:?}", ans);
        // assert_eq!(vec![vec![2,1]], ans);
        candidates = vec![2, 5, 2, 1, 2];
        target = 5;
        ans = Solution::combination_sum2(candidates, target);
        println!("ans: {:?}", ans);
    }
}
