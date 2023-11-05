use std::collections::{hash_map::Entry, HashMap};

const OFFSET: usize = "a".as_bytes()[0] as usize;
impl Solution {
    /// best solution: 1 ms and 2.14 MB
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut ransom_vec = (0..=26).into_iter().map(|_| 0).collect::<Vec<u32>>();
        let mut magazine_vec = ransom_vec.to_owned();

        ransom_note
            .as_bytes()
            .iter()
            .for_each(|byte| ransom_vec[*byte as usize - OFFSET] += 1);

        magazine
            .as_bytes()
            .iter()
            .for_each(|byte| magazine_vec[*byte as usize - OFFSET] += 1);

        ransom_vec.iter().zip(magazine_vec).all(|(r, m)| m >= *r)
    }

    /// this is really fast: 0ms and 2.24 MB
    pub fn can_construct6(ransom_note: String, magazine: String) -> bool {
        const OFFSET: u8 = "a".as_bytes()[0];
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut ransom_vec = (0..=26).into_iter().map(|_| 0).collect::<Vec<u32>>();
        let mut magazine_vec = (0..=26).into_iter().map(|_| 0).collect::<Vec<u32>>();

        ransom_note
            .as_bytes()
            .iter()
            .for_each(|b| ransom_vec[(b - OFFSET) as usize] += 1);
        magazine
            .as_bytes()
            .iter()
            .for_each(|b| magazine_vec[(b - OFFSET) as usize] += 1);

        for (ransom_count, magazine_count) in ransom_vec.iter().zip(magazine_vec) {
            if *ransom_count > magazine_count {
                return false;
            }
        }

        true
    }
    /// this is pretty trash
    pub fn can_construct5(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut sorted_magazine = magazine.as_bytes().iter().collect::<Vec<&u8>>();
        sorted_magazine.sort_unstable();
        sorted_magazine.dedup();
        let mut sorted_ransom = ransom_note.as_bytes().iter().collect::<Vec<&u8>>();
        sorted_ransom.sort_unstable();
        sorted_ransom.dedup();

        let ransom_note_count = sorted_ransom
            .iter()
            .map(|c| (*c, ransom_note.as_bytes().iter().filter(|b| b == c).count()))
            .collect::<HashMap<&u8, usize>>();

        let magazine_count = sorted_magazine
            .iter()
            .map(|c| (*c, magazine.as_bytes().iter().filter(|b| b == c).count()))
            .collect::<HashMap<&u8, usize>>();

        if ransom_note_count.len() > magazine_count.len() {
            return false;
        }

        for (key, value) in ransom_note_count.iter() {
            if !magazine_count.contains_key(key) {
                return false;
            }
            if value > magazine_count.get(key).unwrap() {
                return false;
            }
        }

        true
    }

    /// 6 ms and 2.14MB
    pub fn can_construct4(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut magazine_letters: HashMap<u8, u32> = HashMap::with_capacity(27);
        magazine
            .as_bytes()
            .iter()
            .for_each(|&b| *magazine_letters.entry(b).or_insert(0) += 1);

        let mut ransom_note_letters: HashMap<u8, u32> = HashMap::with_capacity(27);
        ransom_note
            .as_bytes()
            .iter()
            .for_each(|&b| *ransom_note_letters.entry(b).or_insert(0) += 1);

        if ransom_note_letters.len() > magazine_letters.len() {
            return false;
        }

        for (key, value) in ransom_note_letters.iter() {
            if !magazine_letters.contains_key(key) {
                return false;
            }
            if value > magazine_letters.get(key).unwrap() {
                return false;
            }
        }

        true
    }

    /// not faster
    /// 7ms and 2.1 MB
    pub fn can_construct3(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut magazine_letters = HashMap::with_capacity(27);
        magazine
            .chars()
            .for_each(|c| *magazine_letters.entry(c).or_insert(0) += 1);

        let mut ransom_note_letters = HashMap::with_capacity(27);
        ransom_note
            .chars()
            .for_each(|c| *ransom_note_letters.entry(c).or_insert(0) += 1);

        if ransom_note_letters.len() > magazine_letters.len() {
            return false;
        }

        for (key, value) in ransom_note_letters.iter() {
            if !magazine_letters.contains_key(&key) {
                return false;
            }
            if value > magazine_letters.get(&key).unwrap() {
                return false;
            }
        }

        true
    }
    /// much better memory usage
    /// 7ms and 2.05 MB
    pub fn can_construct2(ransom_note: String, magazine: String) -> bool {
        // return early in simple cases
        if ransom_note.len() > magazine.len() {
            return false;
        }

        // make a hashmap of characters to their counts
        let mut magazine_letters = HashMap::with_capacity(27);
        magazine
            .chars()
            .for_each(|c| *magazine_letters.entry(c).or_insert(0) += 1);

        // check if we can write the note with the given magazine
        for character in ransom_note.chars() {
            if let Entry::Occupied(mut count) = magazine_letters.entry(character) {
                *count.get_mut() -= 1;

                if *count.get() < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
    /// 7 ms and 2.28 MB
    /// works, but not very good
    pub fn can_construct1(ransom_note: String, magazine: String) -> bool {
        // make a hashmap of characters to their counts
        let mut magazine_letters = HashMap::new();
        for character in magazine.chars() {
            *magazine_letters.entry(character).or_insert(0) += 1;
        }

        // check if we can write the note with the given magazine
        for character in ransom_note.chars() {
            if let Entry::Occupied(mut count) = magazine_letters.entry(character) {
                *count.get_mut() -= 1;

                if *count.get() < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[test]
fn test1() {
    assert!(!Solution::can_construct(
        String::from("a"),
        String::from("b")
    ));
}

#[test]
fn test2() {
    assert!(!Solution::can_construct(
        String::from("aa"),
        String::from("ab")
    ));
}

#[test]
fn test3() {
    assert!(Solution::can_construct(
        String::from("aa"),
        String::from("aab")
    ));
}

#[test]
fn test4() {
    assert!(Solution::can_construct(
        String::from("ag"),
        String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj")
    ));
}

struct Solution {}

fn main() {}
