use std::collections::HashMap;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].to_owned();
        }

        let mut root = TreeNode::new(37);
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
        println!("{:?}", root);

        if root.children.len() != 1 {
            return "".to_string();
        }

        let mut letters = Vec::new();
        let mut key = root.children.keys().next().unwrap().to_owned();
        let mut node = root.children.get_mut(&key).unwrap();
        while node.children.len() == 1 {
            letters.push(node.letter);
            key = node.children.keys().next().unwrap().to_owned();
            node = node.children.get_mut(&key).unwrap();
        }
        if node.letter != 36 {
            letters.push(node.letter);
        }

        String::from_utf8(letters).unwrap()
    }

    /// very fast at 0 ms but with 2.18 MB
    pub fn longest_common_prefix1(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].to_owned();
        }

        let mut root = TreeNode::new(37);
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
        println!("{:?}", root);

        if root.children.len() != 1 {
            return "".to_string();
        }

        let mut letters = Vec::new();
        let mut key = root.children.keys().next().unwrap().to_owned();
        let mut node = root.children.get_mut(&key).unwrap();
        while node.children.len() == 1 {
            letters.push(node.letter);
            key = node.children.keys().next().unwrap().to_owned();
            node = node.children.get_mut(&key).unwrap();
        }
        if node.letter != 36 {
            letters.push(node.letter);
        }

        String::from_utf8(letters).unwrap()
    }
}

#[derive(Debug)]
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

#[test]
fn ex_5() {
    let strs = vec![
        "flower".into(),
        "flower".into(),
        "flower".into(),
        "flower".into(),
    ];
    assert_eq!("flower".to_string(), Solution::longest_common_prefix(strs));
}

struct Solution {}
fn main() {}
