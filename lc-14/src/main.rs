use std::collections::HashMap;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // this is ok because only lowercase letters are used
        if strs.len() == 1 {
            return strs[0].to_owned();
        }

        let mut root = TreeNode::new(0);

        for mut s in strs {
            if s.is_empty() {
                return "".to_string();
            }
            let mut node = &mut root;
            s.push('$');
            for byte in s.as_bytes() {
                node = node.get_or_add_child(byte);
            }
        }
        // println!("{:?}", root);

        let mut letters = Vec::new();

        let mut node = &mut root;
        while node.children.len() == 1 {
            letters.push(node.letter);
            let key = node.children.keys().next().unwrap().to_owned();
            node = node.children.get_mut(&key).unwrap();
        }
        letters.push(node.letter);

        if letters.len() == 1 {
            "".to_string()
        } else {
            letters.remove(0);
            String::from_utf8(letters).unwrap()
        }
    }
}

// #[derive(Debug)]
struct TreeNode {
    pub letter: u8,
    pub children: HashMap<u8, Self>,
}

impl TreeNode {
    pub fn new(letter: u8) -> Self {
        Self {
            letter,
            children: HashMap::new(),
        }
    }

    pub fn get_or_add_child(&mut self, letter: &u8) -> &mut Self {
        if !self.children.contains_key(letter) {
            self.children.insert(*letter, TreeNode::new(*letter));
        }
        self.children.get_mut(letter).unwrap()
    }
}

#[test]
fn ex_1() {
    let strs = vec!["flower".into(), "flow".into(), "flight".into()];
    assert_eq!("fl".to_string(), Solution::longest_common_prefix(strs));
}

#[test]
fn ex_2() {
    let strs = vec!["dog".into(), "racecar".into(), "car".into()];
    assert_eq!("".to_string(), Solution::longest_common_prefix(strs));
}

#[test]
fn ex_3() {
    let strs = vec!["".into(), "b".into()];
    assert_eq!("".to_string(), Solution::longest_common_prefix(strs));
}

#[test]
fn ex_4() {
    let strs = vec!["ab".into(), "a".into()];
    assert_eq!("a".to_string(), Solution::longest_common_prefix(strs));
}

struct Solution {}
fn main() {}
