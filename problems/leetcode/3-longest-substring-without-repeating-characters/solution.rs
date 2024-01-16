struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    /**
     * this is a solution I saw on solution tab, this is much more idiomatic
     * and I think I figured how it works.
     *
     * first when we insert into the hashmap. It returns None as there was no
     * value before. then we set it as 0 (-1 + 1) which means we still haven't
     * found any duplicates.
     *
     * once we found a duplicate, the map will return Some with the index that
     * was duplicated, so we set our new start to be 1 after that
     * "abcabcbb" -> we find at abc>a< which will return 0
     * "pwwkew" -> we find at pw>w< which will return 1
     * knowing the index that we found the duplicate, we can skip until i+1
     * as i + 1 will always be inside of the range we are currently looking
     *
     * after this, we just set the maximum to be the current index we are on the
     * string, minus the index of the last non-duplicate character + 1 (0 indexed)
     *
     * the max between the current maximum and the current index - start is the
     * answer
     */
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = HashMap::new();
        let mut start = 0;
        let mut max = 0;

        for (idx, ch) in s.chars().enumerate() {
            start = start.max(seen.insert(ch, idx as i32).unwrap_or(-1) + 1);
            max = max.max(idx as i32 - start + 1);
        }
        max
    }

    /**
     * this was the solution I managed to do.
     */
    pub fn length_of_longest_substring_using_set(s: String) -> i32 {
        let mut seen = HashSet::with_capacity(s.len());
        let mut max = 0;

        let mut left = 0;

        for right in 0..s.len() {
            let rc = s.chars().nth(right);
            if !seen.contains(&rc) {
                seen.insert(rc);
            } else {
                while seen.contains(&rc) {
                    let lc = s.chars().nth(left);
                    seen.remove(&lc);
                    left += 1
                }
                seen.insert(rc);
            }

            max = max.max(right - left + 1);
        }

        max as i32
    }
}

fn main() {
    let s1 = String::from("abcabcbb");
    let s2 = String::from("bbbbb");
    let s3 = String::from("pwwkew");

    assert_eq!(Solution::length_of_longest_substring(s1), 3);
    assert_eq!(Solution::length_of_longest_substring(s2), 1);
    assert_eq!(Solution::length_of_longest_substring(s3), 3);
}
