struct Solution;

impl Solution {
    fn backtrace(input: &Vec<u8>, pos: usize, temp: &mut Vec<u8>, ans: &mut Vec<String>) {
        if temp.len() == input.len() {
            let curr = String::from_utf8(temp.clone()).unwrap();
            ans.push(curr);
            return;
        }

        temp.push(input[pos]);
        Solution::backtrace(&input, pos + 1, temp, ans);
        temp.pop();
        if input[pos].is_ascii_lowercase() {
            temp.push(input[pos].to_ascii_uppercase());
            Solution::backtrace(&input, pos + 1, temp, ans);
            temp.pop();
        }
        if input[pos].is_ascii_uppercase() {
            temp.push(input[pos].to_ascii_lowercase());
            Solution::backtrace(&input, pos + 1, temp, ans);
            temp.pop();
        }
    }
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut temp = Vec::new();
        Solution::backtrace(&s.as_bytes().to_vec(), 0, &mut temp, &mut ans);

        return ans.into();
    }
}

#[test]
fn test_letter_case_permutation() {
    let s = "1a".to_string();
    let ans = Solution::letter_case_permutation(s);
    println!("{:?}", ans);
}
