struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut a = num1.as_bytes().to_vec();
        let mut b = num2.as_bytes().to_vec();
        a.reverse();
        b.reverse();
        let mut c = Vec::new();

        let mut c_i = 0;
        let mut carry = 0;

        loop {
            if c_i >= a.len() + b.len() - 1 && carry == 0 {
                break;
            }
            // 计算第c_i位的结果
            let mut v = 0;
            let mut a_i = 0;
            while a_i <= c_i && (a_i < a.len() || c_i - a_i < b.len()) {
                let mut op1 = 0;
                let mut op2 = 0;
                if a_i < a.len() {
                    op1 = (a[a_i] - '0' as u8) as u32;
                }
                if c_i - a_i < b.len() {
                    op2 = (b[c_i - a_i] - '0' as u8) as u32;
                }
                v += op1 * op2;
                a_i += 1;
            }
            // println!("c_i: {:?}, v: {:?}", c_i, v);
            c.push(((v + carry) % 10) as u8 + '0' as u8);
            carry = (v + carry) / 10;
            c_i += 1;
        }

        // 高位去零
        while c.len() > 1 && *c.last().unwrap() == '0' as u8 {
            c.pop();
        }
        c.reverse();
        return std::str::from_utf8(&c).unwrap().to_string();
    }
}

#[test]
fn test_multiply() {
    let a = "2".to_string();
    let b = "3".to_string();
    let mut ans = Solution::multiply(a, b);
    assert_eq!("6".to_string(), ans);
    ans = Solution::multiply("999".to_string(), "0".to_string());
    assert_eq!("0".to_string(), ans);
}
