impl Solution {
    /// super fast, but uses a tiny bit more memory
    pub fn is_valid(s: String) -> bool {
        // if there is an incomplete pair of parens, it has to be wrong
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack = Vec::with_capacity(s.len() / 2);
        for paren in s.chars() {
            match paren {
                '(' | '[' | '{' => stack.push(paren),
                ')' | ']' | '}' => {
                    if let Some(last_opened) = stack.pop() {
                        match (last_opened, paren) {
                            ('(', ')') | ('[', ']') | ('{', '}') => {}
                            (_, _) => {
                                // if the parens don't match, it's wrong
                                return false;
                            }
                        }
                    } else {
                        // there are no open parens but we want to close one
                        return false;
                    }
                }
                _ => unreachable!("there can be no other characters"),
            }
        }

        // if nothing broke, the parens are ok
        stack.len() == 0
    }
}

#[test]
fn ex_1() {
    assert!(Solution::is_valid("()".into()))
}
#[test]
fn ex_2() {
    assert!(Solution::is_valid("()[]{}".into()))
}
#[test]
fn ex_3() {
    assert!(!Solution::is_valid("(]".into()))
}
#[test]
fn ex_4() {
    assert!(Solution::is_valid("([]{()})".into()))
}
#[test]
fn ex_5() {
    assert!(!Solution::is_valid("([{]()})".into()))
}
#[test]
fn ex_6() {
    assert!(!Solution::is_valid(")".into()))
}

struct Solution {}

fn main() {}
