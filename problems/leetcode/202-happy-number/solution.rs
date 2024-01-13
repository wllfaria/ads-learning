struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen: HashMap<i32, bool> = HashMap::new();

        while n != 1 {
            if seen.contains_key(&n) {
                return false;
            }
            seen.insert(n, true);
            n = n.to_string().chars().fold(0, |mut acc, v| {
                let digit = v.to_digit(10).unwrap() as i32;
                acc += digit.pow(2);
                acc
            });
        }
        true
    }
}

fn main() {
    assert_eq!(Solution::is_happy(19), true);
    assert_eq!(Solution::is_happy(2), false);
}
