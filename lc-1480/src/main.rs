struct Solution {}

impl Solution {
    /// this is as fast as the first one but uses 200 kB less RAM
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter()
            .map(|n| {
                sum += n;
                sum
            })
            .collect()
    }

    /// takes 1 ms and 2.2 MB of RAM
    pub fn running_sum1(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[test]
fn test_running_sum() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(nums));

    let nums = vec![1, 1, 1, 1, 1];
    assert_eq!(vec![1, 2, 3, 4, 5], Solution::running_sum(nums));

    let nums = vec![3, 1, 2, 10, 1];
    assert_eq!(vec![3, 4, 6, 16, 17], Solution::running_sum(nums));
}

fn main() {
    println!("Hello, world!");
}
