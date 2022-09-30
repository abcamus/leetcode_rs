use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn bfs(grid: &mut Vec<Vec<i32>>, dis: &mut Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 2 {
                    dis[i][j] = 0;
                    q.push_back((i as i32, j as i32));
                }
            }
        }

        let mut depth = 0;
        let pos_array = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
        while !q.is_empty() {
            let pos = q.pop_front().unwrap();
            for p in &pos_array {
                let x = pos.0 + p.0;
                let y = pos.1 + p.1;
                if x >= 0
                    && x < grid.len() as i32
                    && y >= 0
                    && y < grid[x as usize].len() as i32
                    && grid[x as usize][y as usize] == 1
                    && dis[x as usize][y as usize] < 0
                {
                    // 苹果烂掉
                    grid[x as usize][y as usize] = 2;
                    // 更新当前苹果烂掉时刻
                    dis[x as usize][y as usize] = dis[pos.0 as usize][pos.1 as usize] + 1;
                    // 记录时间
                    depth = dis[x as usize][y as usize];
                    q.push_back((x, y));
                }
            }
        }
        return depth;
    }
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut dis = grid.clone();
        for i in 0..grid.len() {
            dis[i].fill(-1);
        }

        let mut new_grid = grid.clone();
        let depth = Solution::bfs(&mut new_grid, &mut dis);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if new_grid[i][j] == 1 && dis[i][j] < 0 {
                    return -1;
                }
            }
        }
        return depth;
    }
}

#[test]
fn test_orange_rotting() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let ans = Solution::oranges_rotting(grid);
    assert_eq!(4, ans);
}
