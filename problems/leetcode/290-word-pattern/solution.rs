struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_map = HashMap::with_capacity(pattern.len());
        let mut word_map = HashMap::with_capacity(pattern.len());
        let words = s.split(" ").collect::<Vec<_>>();

        if words.len() != pattern.len() {
            return false;
        }

        for (i, p) in pattern.chars().enumerate() {
            let stored_pattern = pattern_map.entry(words[i]).or_insert(p);
            let word = word_map.entry(p).or_insert(words[i]);
            if *word != words[i] || stored_pattern != &p {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string())
    );
    assert_eq!(
        false,
        Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string())
    );
    assert_eq!(
        false,
        Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string())
    );
}
