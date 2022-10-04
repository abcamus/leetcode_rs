struct Solution;
impl Solution {
    fn loop_impl(x: u32) -> u32 {
        let mut ans = 0;
        for i in 0..32 {
            ans |= ((x & (1 << (31 - i))) >> (31 - i)) << i;
        }
        return ans;
    }
    fn divide_impl(x: u32) -> u32 {
        let mut n = (x >> 16) | (x << 16);
        n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
        n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
        n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
        n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
        return n;
    }
    pub fn reverse_bits(x: u32) -> u32 {
        return Solution::divide_impl(x);
    }
}

#[test]
fn test_reverse_bits() {
    assert_eq!(Solution::reverse_bits(43261596), 964176192);
    assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
}
