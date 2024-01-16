struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut min = i32::MAX;
        let mut sum = 0;

        while end < nums.len() {
            sum += nums[end];
            end += 1;

            while start < end && sum >= target {
                sum -= nums[start];
                start += 1;

                min = i32::min(min, end as i32 - start as i32 + 1);
            }
        }

        match min {
            i32::MAX => 0,
            _ => min,
        }
    }
}

fn main() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}
