struct Solution;

impl Solution {
    fn reverse_string(s: &mut [u8]) {
        let mut left = 0 as i32;
        let mut right = s.len() as i32 - 1;
        while left <= right {
            s.swap(left as usize, right as usize);
            left += 1;
            right -= 1;
        }
    }
    pub fn reverse_words(s: String) -> String {
        let mut v = s.into_bytes().to_vec();
        for sub_str in v.split_mut(|a| *a == ' ' as u8) {
            Solution::reverse_string(sub_str);
        }

        return std::str::from_utf8(&v).unwrap().to_string();
    }
}
