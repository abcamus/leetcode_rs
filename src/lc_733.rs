struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn bfs(image: &mut Vec<Vec<i32>>, x: usize, y: usize, color: i32) -> Vec<Vec<i32>> {
        let old = image[x][y];
        let mut q = VecDeque::new();
        q.push_back((x, y));
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            image[x][y] = color;
            if x > 0 && image[x - 1][y] == old {
                q.push_back((x - 1, y));
            }
            if x + 1 < image.len() && image[x + 1][y] == old {
                q.push_back((x + 1, y));
            }
            if y > 0 && image[x][y - 1] == old {
                q.push_back((x, y - 1));
            }
            if y + 1 < image[x].len() && image[x][y + 1] == old {
                q.push_back((x, y + 1));
            }
        }

        return image.clone();
    }
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        if image[sr as usize][sc as usize] == color {
            return image;
        }
        return Solution::bfs(&mut image.clone(), sr as usize, sc as usize, color);
    }
}
