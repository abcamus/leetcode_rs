struct Solution;
impl Solution {
    // real len of nums1 is m+n
    fn use_sort(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for idx in 0 as usize..n as usize {
            nums1[idx + m as usize] = nums2[idx];
        }
        nums1.sort();
    }
    fn double_pointer(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx = m + n - 1;
        let mut idx1: i32 = m - 1;
        let mut idx2: i32 = n - 1;
        while idx >= 0 {
            if idx1 < 0 {
                nums1[idx as usize] = nums2[idx2 as usize];
                idx2 -= 1;
            } else if idx2 < 0 {
                nums1[idx as usize] = nums1[idx1 as usize];
                idx1 -= 1;
            } else {
                // 都有效
                if nums1[idx1 as usize] < nums2[idx2 as usize] {
                    nums1[idx as usize] = nums2[idx2 as usize];
                    idx2 -= 1;
                } else {
                    nums1[idx as usize] = nums1[idx1 as usize];
                    idx1 -= 1;
                }
            }
            idx -= 1;
        }
    }
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        return Solution::double_pointer(nums1, m, nums2, n);
    }
}
