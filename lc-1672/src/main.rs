struct Solution {}

impl Solution {
    /// takes 1ms and 2.09 MB
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_val = 0;
        for account in accounts {
            let sum = account.iter().sum();
            max_val = max_val.max(sum);
        }
        max_val
    }

    /// works, takes 1 ms and 2.34 MB
    pub fn maximum_wealth2(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .expect("array cannot be empty")
    }

    /// works, takes 1 ms and 2.24 MB
    pub fn maximum_wealth1(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .fold(0, |max_val, account| max_val.max(account.iter().sum()))
    }
}

#[test]
fn test_maximum_wealth() {
    let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
    assert_eq!(6, Solution::maximum_wealth(accounts));

    let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    assert_eq!(10, Solution::maximum_wealth(accounts));

    let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
    assert_eq!(17, Solution::maximum_wealth(accounts));
}

fn main() {}
