struct Solution {}

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        let mut end: i32 = -1;
        let mut can_attend = true;
        intervals.sort();
        intervals.iter().for_each(|x| {
            if end.is_negative() || x[0] >= end {
                end = x[1];
            } else {
                can_attend = false;
            }
        });
        can_attend
    }
}

fn main() {
    let intervals_one = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    let intervals_two = vec![vec![7, 10], vec![2, 4]];
    let intervals_three = vec![vec![0, 30], vec![60, 240], vec![90, 120]];

    let result_one = Solution::can_attend_meetings(intervals_one);
    let result_two = Solution::can_attend_meetings(intervals_two);
    let result_three = Solution::can_attend_meetings(intervals_three);

    assert_eq!(result_one, false);
    assert_eq!(result_two, true);
    assert_eq!(result_three, false);
}
