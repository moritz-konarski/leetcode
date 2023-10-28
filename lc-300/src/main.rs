struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: &Vec<i32>) -> i32 {
        let mut diffs = Vec::with_capacity(nums.len() - 1);
        for i in 0..(nums.len() - 1) {
            diffs.push(nums[i + 1] - nums[i]);
        }
        // let diffs = nums.windows(2).for_each(|window| {
        //     window
        //         .iter()
        //         .reduce(|diff, n| diff - n)
        //         .expect("we always have two values")
        // });
        eprintln!("{:?}", diffs);
        diffs.len() as i32
    }
}

#[test]
fn test_examples() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(4, Solution::length_of_lis(&nums));

    let nums = vec![0, 1, 0, 3, 2, 3];
    assert_eq!(4, Solution::length_of_lis(&nums));

    let nums = vec![7, 7, 7, 7, 7, 7, 7];
    assert_eq!(1, Solution::length_of_lis(&nums));
}

fn main() {}
