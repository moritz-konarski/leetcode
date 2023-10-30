struct Solution {}

impl Solution {
    /// 2ms, 2.57 MB
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|num| match (num % 3, num % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                (_, _) => num.to_string(),
            })
            .collect::<Vec<String>>()
    }

    /// 2ms, 2.72 MB
    pub fn fizz_buzz3(n: i32) -> Vec<String> {
        (1..=n)
            .map(|num| {
                let mut s = String::new();

                if num % 3 == 0 {
                    s.push_str("Fizz");
                }

                if num % 5 == 0 {
                    s.push_str("Buzz");
                }

                if s.is_empty() {
                    s.push_str(num.to_string().as_str());
                }

                s
            })
            .collect::<Vec<String>>()
    }
    /// 2ms, 2.70 MB
    pub fn fizz_buzz2(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut output = (1..=n).map(|num| num.to_string()).collect::<Vec<String>>();

        // get Fizz
        for i in (3..=n).step_by(3) {
            output[i - 1] = "Fizz".to_string();
        }

        // get Buzz
        for i in (5..=n).step_by(5) {
            if i % 3 == 0 {
                output[i - 1].push_str("Buzz");
            } else {
                output[i - 1] = "Buzz".to_string();
            }
        }

        output
    }
    /// 2 ms, 2.66 MB
    pub fn fizz_buzz1(n: i32) -> Vec<String> {
        let mut output = Vec::<String>::with_capacity(n as usize);
        for i in 1..=n {
            match i {
                i if i % 15 == 0 => output.push("FizzBuzz".to_string()),
                i if i % 3 == 0 => output.push("Fizz".to_string()),
                i if i % 5 == 0 => output.push("Buzz".to_string()),
                _ => output.push(i.to_string()),
            }
        }
        output
    }
}

#[test]
fn test_fizz_buzz() {
    let n = 3;
    assert_eq!(vec!["1", "2", "Fizz"], Solution::fizz_buzz(n));

    let n = 5;
    assert_eq!(vec!["1", "2", "Fizz", "4", "Buzz"], Solution::fizz_buzz(n));

    let n = 15;
    assert_eq!(
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ],
        Solution::fizz_buzz(n)
    );
}

fn main() {}
