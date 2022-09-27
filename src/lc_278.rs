// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct Solution;

impl Solution {
    fn isBadVersion(&self, n: i32) -> bool {
        unimplemented!()
    }
    fn binary_find(&self, mut left: i64, mut right: i64) -> i32 {
        let mut min = right;
        while left <= right {
            let mid = (left + right) / 2;
            if self.isBadVersion(mid as i32) {
                min = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return min as i32;
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        return self.binary_find(1, n as i64);
    }
}
