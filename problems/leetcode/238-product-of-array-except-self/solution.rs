struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![];
        for i in 0..nums.len() {
            match i {
                0 => answer.push(1),
                _ => answer.push(nums[i - 1] * answer[i - 1]),
            }
        }
        let mut right: i32 = 1;
        for i in (0..nums.len()).rev() {
            answer[i] = right * answer[i];
            right *= nums[i];
        }
        answer
    }
}

fn main() {
    let nums_1 = vec![1, 2, 3, 4];
    let nums_2 = vec![-1, 1, 0, -3, 3];

    assert_eq!(Solution::product_except_self(nums_1), vec![24, 12, 8, 6]);
    assert_eq!(Solution::product_except_self(nums_2), vec![0, 0, 9, 0, 0]);
}
