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
        op1.push(carry + '0' as u8);
    }

    op1.reverse();

    return std::str::from_utf8(&op1).unwrap().to_string();
}

pub fn binary_search(nums: &Vec<i32>, mut left: i32, mut right: i32, target: i32) -> i32 {
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return right + 1;
}

#[test]
fn test_vector_append() {
    let mut a = vec![1, 2];
    let mut b = vec![3, 4];
    a.append(&mut b);
    assert_eq!(vec![1, 2, 3, 4], a);
}
