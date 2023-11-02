struct Solution {}

impl Solution {
    /// 1 ms, 1.93 MB
    pub fn number_of_steps(mut num: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        let mut steps = -1;
        while num > 0 {
            steps += 1 + (num & 1);
            num >>= 1;
        }
        steps
    }

    /// works, but the Leetcode compiler is too old
    pub fn number_of_steps3(num: i32) -> i32 {
        if let Some(bit_count) = num.checked_ilog2() {
            let uneven_count = num.count_ones();
            (bit_count + uneven_count) as i32
        } else {
            0
        }
    }

    /// 1 ms, 2.1 MB
    pub fn number_of_steps2(mut num: i32) -> i32 {
        let mut steps = 0;

        while num > 0 {
            if num % 2 == 0 {
                steps += 1;
            } else {
                steps += 2;
            }
            num /= 2;
        }

        if steps == 0 {
            steps = 1;
        }
        steps
    }

    /// takes 1ms and uses 2.03 MB
    pub fn number_of_steps1(mut num: i32) -> i32 {
        let mut steps = 0;
        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            steps += 1;
        }
        steps
    }
}

#[test]
fn test() {
    assert_eq!(6, Solution::number_of_steps(14));

    assert_eq!(4, Solution::number_of_steps(8));

    assert_eq!(12, Solution::number_of_steps(123));

    assert_eq!(0, Solution::number_of_steps(0));
}

fn main() {}
