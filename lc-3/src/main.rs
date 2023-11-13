use std::collections::HashMap;

// all characters from 32 (space) up to 126 (~) are allowed
// instead of hashing, we can create an array
const ARRAY_LEN: usize = 95;
const OFFSET: usize = 32;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            _ => {
                let mut running_len = 0;
                let mut max = 0;
                let mut last_index_vec: Vec<i32> = Vec::with_capacity(ARRAY_LEN);
                (0..ARRAY_LEN)
                    .into_iter()
                    .for_each(|_| last_index_vec.push(-1));

                for (i, b) in s.bytes().enumerate() {
                    let index: usize = b as usize - OFFSET;
                    if last_index_vec[index] != -1 {
                        let len = (i as i32) - last_index_vec[index];
                        max = max.max(len);
                        running_len = 0;
                    }
                    running_len += 1;
                    last_index_vec[index] = i as i32;
                    max = max.max(running_len)
                }

                max
            }
        }
    }

    /// still 3 ms and now 2.13MB
    pub fn length_of_longest_substring6(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            n => {
                let mut max = 0;
                let mut count = 0;
                let mut bit_vec: Vec<bool> = Vec::with_capacity(ARRAY_LEN);
                (0..ARRAY_LEN).into_iter().for_each(|_| bit_vec.push(false));

                for start_index in (0..n).into_iter() {
                    bit_vec.iter_mut().for_each(|b| *b = false);
                    for b in s.bytes().skip(start_index) {
                        let index: usize = b as usize - OFFSET;
                        if bit_vec[index] {
                            break;
                        }

                        bit_vec[index] = true;
                        count += 1;
                    }
                    max = max.max(count);
                    count = 0;
                }

                max
            }
        }
    }
    /// much faster at 74ms and 2.15 MB
    pub fn length_of_longest_substring5(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            n => {
                let mut max = 0;
                let mut count = 0;
                let mut map: HashMap<u8, u8> = HashMap::new();

                for start_index in (0..n).into_iter() {
                    for b in s.bytes().skip(start_index) {
                        if map.contains_key(&b) {
                            break;
                        }

                        map.insert(b, 1);
                        count += 1;
                    }
                    max = max.max(count);
                    count = 0;
                    map.clear();
                }

                max
            }
        }
    }
    /// SUPER slow at 126 ms, but good on memory at 2.18 MB
    pub fn length_of_longest_substring4(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            n => {
                let mut max = 0;
                let mut count = 0;

                for start_index in (0..n).into_iter() {
                    let mut map: HashMap<u8, u8> = HashMap::new();
                    for b in s.bytes().skip(start_index) {
                        if map.contains_key(&b) {
                            break;
                        }

                        map.insert(b, 1);
                        count += 1;
                    }
                    max = max.max(count);
                    count = 0;
                }

                max
            }
        }
    }
    /// this is much faster again
    /// 3ms and 2.26 MB
    pub fn length_of_longest_substring3(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            n => {
                let mut max = 0;
                let mut count = 0;
                let mut bit_vec: Vec<bool> = (0..ARRAY_LEN).into_iter().map(|_| false).collect();

                for start_index in (0..n).into_iter() {
                    bit_vec.iter_mut().for_each(|b| *b = false);
                    for b in s.bytes().skip(start_index) {
                        let index: usize = b as usize - OFFSET;
                        if bit_vec[index] {
                            break;
                        }

                        bit_vec[index] = true;
                        count += 1;
                    }
                    max = max.max(count);
                    count = 0;
                }

                max
            }
        }
    }
    /// MUCH faster because of the correct max placement
    /// 7 ms and 2.27 MB
    pub fn length_of_longest_substring2(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            n => {
                let mut max = 0;
                let mut bit_vec: Vec<bool> = (0..ARRAY_LEN).into_iter().map(|_| false).collect();

                for start_index in (0..n).into_iter() {
                    bit_vec.iter_mut().for_each(|b| *b = false);
                    for b in s.bytes().skip(start_index) {
                        let index: usize = b as usize - OFFSET;
                        if bit_vec[index] {
                            break;
                        }

                        bit_vec[index] = true;
                    }
                    max = max.max(bit_vec.iter().map(|b| *b as i32).sum());
                }

                max
            }
        }
    }
    /// first working version: 46 ms and 2.30 MB
    pub fn length_of_longest_substring1(s: String) -> i32 {
        match s.len() {
            0 => 0,
            1 => 1,
            2 => {
                return (s.as_bytes()[0] != s.as_bytes()[1]) as i32 + 1;
            }
            n => {
                let mut max = 0;
                let mut bit_vec: Vec<bool> = (0..ARRAY_LEN).into_iter().map(|_| false).collect();

                // we can treat this as bytes because it is only ASCII
                for start_index in (0..n).into_iter() {
                    bit_vec.iter_mut().for_each(|b| *b = false);
                    for b in s.bytes().skip(start_index) {
                        let index: usize = b as usize - OFFSET;
                        if bit_vec[index] {
                            break;
                        } else {
                            bit_vec[index] = true;
                        }
                        max = max.max(bit_vec.iter().map(|b| *b as i32).sum());
                    }
                }

                max
            }
        }
    }
}

#[test]
fn test1() {
    eprintln!("'abcabcbb' gets 'abc' with length 3");
    assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".into()));
}

#[test]
fn test2() {
    eprintln!("'bbbbb' gets 'b' with length 1");
    assert_eq!(1, Solution::length_of_longest_substring("bbbbb".into()));
}

#[test]
fn test3() {
    eprintln!("'pwwkew' gets 'wke' with length 3");
    assert_eq!(3, Solution::length_of_longest_substring("pwwkew".into()));
}

#[test]
fn test4() {
    eprintln!("'empty string' gets 'nothing' with length 0");
    assert_eq!(0, Solution::length_of_longest_substring("".into()));
}

#[test]
fn test5() {
    eprintln!("'a' gets 'a' with length 1");
    assert_eq!(1, Solution::length_of_longest_substring("a".into()));
}

#[test]
fn test6() {
    eprintln!("'au' gets 'au' with length 2");
    assert_eq!(2, Solution::length_of_longest_substring("au".into()));
}

#[test]
fn test7() {
    eprintln!("'aab' gets 'ab' with length 2");
    assert_eq!(2, Solution::length_of_longest_substring("aab".into()));
}

#[test]
fn test8() {
    eprintln!("'cdd' gets 'cd' with length 2");
    assert_eq!(2, Solution::length_of_longest_substring("cdd".into()));
}

struct Solution {}

fn main() {}
