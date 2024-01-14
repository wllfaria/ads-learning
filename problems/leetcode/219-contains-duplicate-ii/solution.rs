struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = HashMap::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate() {
            let idx = seen.entry(v).or_insert(i);
            if *idx != i && i.abs_diff(*idx) as i32 <= k {
                return true;
            }
            if i != *idx {
                *idx = i
            }
        }
        false
    }
}

fn main() {
    let nums_a = vec![1, 2, 3, 1];
    let nums_b = vec![1, 0, 1, 1];
    let nums_c = vec![1, 2, 3, 1, 2, 3];
    assert_eq!(Solution::contains_nearby_duplicate(nums_a, 3), true);
    assert_eq!(Solution::contains_nearby_duplicate(nums_b, 1), true);
    assert_eq!(Solution::contains_nearby_duplicate(nums_c, 2), false);
}
