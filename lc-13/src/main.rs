impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .rev()
            .map(|b| match b {
                73 => 1,
                86 => 5,
                88 => 10,
                76 => 50,
                62 => 100,
                68 => 500,
                77 => 1000,
                _ => unreachable!("not roman numeral"),
            })
            .collect::<Vec<i32>>()
            .as_slice()
            .windows(2)
            .for_each(|a| println!("{:?}", a));
        0
        // .map(|a| if a[0] <= a[1] { a[0] } else { -a[0] })
        // .sum()
    }

    /// super fast but not very memory efficient
    /// 0ms and 2.25 MB
    pub fn roman_to_int1(s: String) -> i32 {
        let mut number = 0;
        let mut val;
        let mut last_val = 0;

        for c in s.chars().into_iter().rev() {
            val = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!("not roman numeral"),
            };

            if val >= last_val {
                number += val;
            } else {
                number -= val;
            }
            last_val = val;
        }

        number
    }
}

#[test]
fn ex_1() {
    assert_eq!(3, Solution::roman_to_int("III".into()));
}

#[test]
fn ex_2() {
    assert_eq!(58, Solution::roman_to_int("LVIII".into()));
}

#[test]
fn ex_3() {
    assert_eq!(1994, Solution::roman_to_int("MCMXCIV".into()));
}

#[test]
fn ex_4() {
    assert_eq!(9, Solution::roman_to_int("IX".into()));
}

struct Solution {}

fn main() {}
