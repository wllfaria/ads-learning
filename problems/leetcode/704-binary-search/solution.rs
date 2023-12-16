use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut hi = nums.len();

        while low < hi {
            let mid = (low + hi) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => hi = mid,
                Ordering::Equal => return mid as i32,
            }
        }

        -1
    }
}

fn main() {
    let nums_1 = vec![-1, 0, 3, 5, 9, 12];
    let target_1 = 9;
    let nums_2 = vec![-1, 0, 3, 5, 9, 12];
    let target_2 = 2;

    assert_eq!(Solution::search(nums_1, target_1), 4);
    assert_eq!(Solution::search(nums_2, target_2), -1);
}
