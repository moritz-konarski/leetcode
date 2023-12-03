impl Solution {
    /// this solution is awesome
    /// 0 ms and 1.96 MB
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        match x {
            0..=9 => true,
            n => {
                let digits = n.to_string();
                let digit_count = digits.len();
                let half_len = digit_count / 2;

                for i in 0..half_len {
                    if digits.as_bytes()[i] != digits.as_bytes()[digit_count - i - 1] {
                        return false;
                    }
                }
                true
            }
        }
    }
}

#[test]
fn ex_1() {
    assert!(Solution::is_palindrome(121));
}

#[test]
fn ex_2() {
    assert!(!Solution::is_palindrome(-121));
}

#[test]
fn ex_3() {
    assert!(!Solution::is_palindrome(10));
}

#[test]
fn ex_4() {
    assert!(Solution::is_palindrome(10101));
}

#[test]
fn ex_5() {
    assert!(Solution::is_palindrome(98789));
}

#[test]
fn ex_6() {
    assert!(!Solution::is_palindrome(987891));
}

#[test]
fn ex_7() {
    assert!(Solution::is_palindrome(987789));
}

#[test]
fn ex_8() {
    assert!(!Solution::is_palindrome(9877891));
}

#[test]
fn ex_9() {
    assert!(!Solution::is_palindrome(122));
}

struct Solution {}
fn main() {}
