use std::collections::HashMap;

impl Solution {
    /// this works but is slow: hashmaps seem to rule
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 2 {
            return vec![0, 1];
        }

        let mut needed_vals = nums
            .iter()
            .enumerate()
            .map(|(i, n)| (-(n - target), i as i32))
            .collect::<Vec<(i32, i32)>>();
        needed_vals.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut nums = nums
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i as i32))
            .collect::<Vec<(i32, i32)>>();
        nums.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        for (num, index) in nums.iter() {
            if let Some(pos) = needed_vals
                .iter()
                .position(|(needed, i)| *needed == *num && *i != *index)
            {
                return vec![*index, needed_vals[pos].1];
            }
        }

        Vec::new()
    }

    /// slow as hell but memory efficient
    /// 33 ms and 2.12 MB
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 2 {
            return vec![0, 1];
        }

        let needed_vec = nums.iter().map(|n| -(n - target)).collect::<Vec<i32>>();
        let res = nums
            .iter()
            .enumerate()
            .filter_map(|(index, num)| {
                if let Some(pos) = needed_vec.iter().position(|needed| needed == num) {
                    if pos != index {
                        Some(index as i32)
                    } else {
                        if let Some(pos) = needed_vec[(pos + 1)..]
                            .iter()
                            .position(|needed| needed == num)
                        {
                            if pos != index {
                                Some(index as i32)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>();

        println!("{:?}", res);

        // let mut num_to_index = HashMap::new();

        // for (index, num) in nums.iter().enumerate() {
        //     let needed = -(num - target);

        //     if num_to_index.contains_key(&needed) {
        //         return vec![*num_to_index.get(&needed).unwrap(), index as i32];
        //     }

        //     num_to_index.insert(num, index as i32);
        // }

        res
    }
    /// pretty fast but not very efficient
    /// 2 ms and 2.55 MB
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 2 {
            return vec![0, 1];
        }

        let mut num_to_index = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let needed = -(num - target);

            if num_to_index.contains_key(&needed) {
                return vec![*num_to_index.get(&needed).unwrap(), index as i32];
            }

            num_to_index.insert(num, index as i32);
        }

        Vec::new()
    }
}

#[test]
fn test1() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
}

#[test]
fn test2() {
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
}

#[test]
fn test3() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
}

#[test]
fn test4() {
    assert_eq!(vec![0, 2], Solution::two_sum(vec![3, 2, 3], 6));
}

struct Solution {}

fn main() {}
