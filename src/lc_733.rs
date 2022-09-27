struct Solution;

impl Solution {
    fn bfs(image: &mut Vec<Vec<i32>>, x: usize, y: usize, color: i32) -> Vec<Vec<i32>> {
        let q = Vec::new();
        q.push((x, y));
        while !q.is_empty() {
            let mut x = q.last().unwrap().0;
            let mut y = q.last().unwrap().1;
            image[x][y] = color;
            if x > 0 && image[x - 1][y] == image[x][y] {
                q.push((x - 1, y));
            }
            if x + 1 < image.len() && image[x + 1][y] == image[x][y] {
                q.push((x + 1, y));
            }
            if y > 0 && image[x][y - 1] == image[x][y] {
                q.push((x, y - 1));
            }
            if y + 1 < image.len() && image[x][y + 1] == image[x][y] {
                q.push((x, y + 1));
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
