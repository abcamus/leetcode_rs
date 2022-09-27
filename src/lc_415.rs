struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (mut op1, mut op2) = if num1.len() > num2.len() {
            (num1.into_bytes(), num2.into_bytes())
        } else {
            (num2.into_bytes(), num1.into_bytes())
        };

        op1.reverse();
        op2.reverse();
        // 双指针
        let mut pos1 = 0;
        let mut pos2 = 0;
        let mut carry = 0;
        while pos2 < op2.len() {
            let value = op1[pos1] - '0' as u8 + op2[pos2] - '0' as u8 + carry;
            op1[pos1] = value % 10 + '0' as u8;
            carry = value / 10;
            pos1 += 1;
            pos2 += 1;
        }

        while pos1 < op1.len() {
            let value = op1[pos1] - '0' as u8 + carry;
            op1[pos1] = value % 10 + '0' as u8;
            carry = value / 10;
            pos1 += 1;
        }

        if carry > 0 {
            op1.push(carry);
        }

        op1.reverse();

        return std::str::from_utf8(&op1).unwrap().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_strings() {
        let mut num1 = "11".to_string();
        let mut num2 = "123".to_string();
        let mut ans = Solution::add_strings(num1, num2);
        assert_eq!("134".to_string(), ans);
    }
}
