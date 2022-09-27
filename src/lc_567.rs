struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let a = s1.as_bytes().to_vec();
        let b = s2.as_bytes().to_vec();
        if a.len() > b.len() {
            return false;
        }

        let mut vec_cnt_a = vec![0; 26];
        let mut vec_cnt_b = vec![0; 26];
        for idx in 0..a.len() {
            vec_cnt_a[a[idx] as usize - 'a' as usize] += 1;
            vec_cnt_b[b[idx] as usize - 'a' as usize] += 1;
        }

        if vec_cnt_a == vec_cnt_b {
            return true;
        }

        for offset in a.len()..b.len() {
            vec_cnt_b[b[offset] as usize - 'a' as usize] += 1;
            vec_cnt_b[b[offset - a.len()] as usize - 'a' as usize] -= 1;
            if vec_cnt_a.eq(&vec_cnt_b) {
                return true;
            }
        }
        return false;
    }
}
