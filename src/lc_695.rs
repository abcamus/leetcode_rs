struct Solution;

impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        let mut cnt = 0;
        let mut s = Vec::new();
        s.push((x, y));
        grid[x][y] = 2;
        while !s.is_empty() {
            cnt += 1;
            let (x, y) = s.pop().unwrap();
            grid[x][y] = 2;
            println!("{:?}", (x, y));
            if x > 0 && grid[x - 1][y] == 1 {
                s.push((x - 1, y));
                grid[x - 1][y] = 2;
            }
            if x + 1 < grid.len() && grid[x + 1][y] == 1 {
                s.push((x + 1, y));
                grid[x + 1][y] = 2;
            }
            if y > 0 && grid[x][y - 1] == 1 {
                s.push((x, y - 1));
                grid[x][y - 1] = 2;
            }
            if y + 1 < grid[x].len() && grid[x][y + 1] == 1 {
                s.push((x, y + 1));
                grid[x][y + 1] = 2;
            }
        }
        return cnt;
    }
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut new_grid = grid.clone();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if new_grid[i][j] == 1 {
                    let curr = Solution::dfs(&mut new_grid, i, j);
                    max = if curr > max { curr } else { max };
                }
            }
        }
        return max;
    }
}
