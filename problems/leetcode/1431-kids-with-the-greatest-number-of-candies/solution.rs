struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        let mut ans = Vec::<bool>::with_capacity(candies.len());

        candies.iter().for_each(|x| {
            ans.push(x + extra_candies >= max);
        });

        ans
    }
}

fn main() {
    let candies_1 = vec![2, 3, 5, 1, 3];
    let extra_candies_1 = 3;
    let candies_2 = vec![4, 2, 1, 1, 2];
    let extra_candies_2 = 1;
    let candies_3 = vec![12, 1, 12];
    let extra_candies_3 = 10;

    assert_eq!(
        Solution::kids_with_candies(candies_1, extra_candies_1),
        vec![true, true, true, false, true]
    );
    assert_eq!(
        Solution::kids_with_candies(candies_2, extra_candies_2),
        vec![true, false, false, false, false]
    );
    assert_eq!(
        Solution::kids_with_candies(candies_3, extra_candies_3),
        vec![true, false, true]
    );
}
