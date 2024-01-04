use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut equivalent: HashMap<char, char> = HashMap::new();
        equivalent.insert(')', '(');
        equivalent.insert('}', '{');
        equivalent.insert(']', '[');
        let mut pairs: Vec<char> = vec![];

        for v in s.chars() {
            if equivalent.contains_key(&v) {
                let last_pair = pairs.pop().unwrap_or('-');
                if last_pair != equivalent[&v] {
                    return false;
                }
                continue;
            }
            pairs.push(v);
        }

        if pairs.len() > 0 {
            return false;
        } else {
            return true;
        }
    }
}

fn main() {
    let s = "()".to_string();
    let s2 = "()[]{}".to_string();
    let s3 = "(]".to_string();
    let s4 = "(".to_string();
    let s5 = "(()".to_string();

    assert_eq!(true, Solution::is_valid(s));
    assert_eq!(true, Solution::is_valid(s2));
    assert_eq!(false, Solution::is_valid(s3));
    assert_eq!(false, Solution::is_valid(s4));
    assert_eq!(false, Solution::is_valid(s5));
}
