struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len: i32 = nums.len() as i32;
        let expected = len * (len + 1) / 2;
        let sum = nums.iter().fold(0, |acc, x| acc + x);
        expected - sum
    }
}

fn main() {
    let nums_1 = vec![3, 0, 1];
    let nums_2 = vec![0, 1];
    let nums_3 = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];

    let answer_1 = Solution::missing_number(nums_1);
    let answer_2 = Solution::missing_number(nums_2);
    let answer_3 = Solution::missing_number(nums_3);

    assert_eq!(answer_1, 2);
    assert_eq!(answer_2, 2);
    assert_eq!(answer_3, 8);
}
