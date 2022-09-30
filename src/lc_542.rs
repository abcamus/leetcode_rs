use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn bfs(mat: &Vec<Vec<i32>>, ans: &mut Vec<Vec<i32>>) {
        let p_vec = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
        let mut q = VecDeque::new();
        for x in 0..mat.len() {
            for y in 0..mat[x].len() {
                if mat[x][y] == 0 {
                    q.push_back((x as i32, y as i32));
                    ans[x][y] = 0;
                }
            }
        }

        while !q.is_empty() {
            let pos = q.pop_front().unwrap();
            for diff in &p_vec {
                let new_x = pos.0 + diff.0;
                let new_y = pos.1 + diff.1;
                if new_x >= 0
                    && new_x < mat.len() as i32
                    && new_y >= 0
                    && new_y < mat[new_x as usize].len() as i32
                    && ans[new_x as usize][new_y as usize] < 0
                {
                    ans[new_x as usize][new_y as usize] = ans[pos.0 as usize][pos.1 as usize] + 1;
                    q.push_back((new_x, new_y));
                }
            }
        }
    }
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = mat.clone();
        for v in ans.iter_mut() {
            v.fill(-1);
        }
        Solution::bfs(&mat, &mut ans);

        return ans.clone();
    }
}
