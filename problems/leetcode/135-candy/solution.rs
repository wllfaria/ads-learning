/*
 * I had absolutely no idea how to solve this so I copied this
 * solution from someone else and tried to understand it as much
 * as I could.
 *
 * future me refer to these:
 * - https://leetcode.com/problems/candy/solutions/4037646/99-20-greedy-two-one-pass/?envType=study-plan-v2&envId=top-interview-150
 * - https://www.youtube.com/watch?v=_MVFeqfiDK4
 */

struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut ret = 1;
        let mut up = 0;
        let mut down = 0;
        let mut peak = 0;

        for i in 0..ratings.len() - 1 {
            let (prev, curr) = (ratings[i], ratings[i + 1]);

            if prev < curr {
                up += 1;
                down = 0;
                peak = up;
                ret += 1 + up;
            } else if prev == curr {
                up = 0;
                down = 0;
                peak = 0;
                ret += 1;
            } else {
                up = 0;
                down += 1;
                ret += 1 + down;
                if peak >= down {
                    ret -= 1;
                }
            }
        }

        ret
    }
}

fn main() {
    let ratings_a = vec![1, 0, 2];
    let ratings_b = vec![1, 0, 2];

    assert_eq!(Solution::candy(ratings_a), 5);
    assert_eq!(Solution::candy(ratings_b), 4);
}
