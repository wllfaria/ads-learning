struct Solution {}

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut single: i32 = 0;
        for i in (0..nums.len()).step_by(2) {
            if nums.len() == i + 1 || nums[i] != nums[i + 1] {
                single = nums[i];
                break;
            }
        }
        single
    }
}

fn main() {
    let nums = vec![4, 1, 2, 1, 2];
    let single = Solution::single_number(nums);
    assert_eq!(single, 4);
}
