struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut data = intervals.clone();
        data.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = vec![];
        let mut curr_region = data[0].clone();
        for i in &data {
            // exceed right side
            let right_side = curr_region[1];
            if i[0] > right_side {
                result.push(curr_region.clone());
                curr_region = i.clone();
            } else {
                // try to expand current region
                if i[1] > right_side {
                    curr_region = vec![curr_region[0], i[1]];
                }
            }
        }
        result.push(curr_region);
        result
    }
}

#[test]
fn test_sort() {
    let mut data = vec![vec![1, 3], vec![4, 6], vec![1, 10], vec![15, 18]];
    data.sort_by(|a, b| a[0].cmp(&b[0]).reverse());
    // println!("{:?}", data);

    let result = Solution::merge(data);
    println!("{:?}", result);
}
