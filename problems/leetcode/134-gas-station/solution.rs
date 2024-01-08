struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut curr_gain = 0;
        let mut total_gain = 0;
        let mut answer = 0;

        for i in 0..gas.len() {
            total_gain += gas[i] - cost[i];
            curr_gain += gas[i] - cost[i];

            if curr_gain < 0 {
                curr_gain = 0;
                answer = i + 1;
            }
        }

        if total_gain < 0 {
            return -1;
        }

        answer as i32
    }
}

fn main() {
    assert_eq!(
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
        -1
    );
    assert_eq!(
        Solution::can_complete_circuit(vec![3, 3, 4], vec![3, 4, 4]),
        -1
    );
}
