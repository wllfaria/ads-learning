struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut seen = HashSet::with_capacity(nums.len());
        let mut max = 0;
        for n in nums {
            seen.insert(n);
        }
        for n in seen.iter() {
            if !seen.contains(&(n - 1)) {
                let mut curr = *n;
                let mut streak = 1;

                while seen.contains(&(curr + 1)) {
                    curr += 1;
                    streak += 1;
                }

                max = max.max(streak);
            }
        }
        max
    }
}

fn main() {
    let nums_a = vec![100, 4, 200, 1, 3, 2];
    let nums_b = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];

    assert_eq!(Solution::longest_consecutive(nums_a), 4);
    assert_eq!(Solution::longest_consecutive(nums_b), 9);
}
