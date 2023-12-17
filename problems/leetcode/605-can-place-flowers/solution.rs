struct Solution {}

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut spots_available = 0;
        let mut prev = 0;

        let mut i = 0;

        while i < flowerbed.len() {
            let next = if i == flowerbed.len() - 1 {
                0
            } else {
                flowerbed[i + 1]
            };

            if flowerbed[i] == 0 && prev == 0 && next == 0 {
                spots_available += 1;
                flowerbed[i] = 1;
                i += 2;
                prev = next;

                continue;
            }

            prev = flowerbed[i];
            i += 1
        }

        spots_available >= n
    }
}

fn main() {
    let flowerbed_1 = vec![1, 0, 0, 0, 1];
    let n_1 = 1;
    let flowerbed_2 = vec![1, 0, 0, 0, 0, 1];
    let n_2 = 2;

    assert_eq!(Solution::can_place_flowers(flowerbed_1, n_1), true);
    assert_eq!(Solution::can_place_flowers(flowerbed_2, n_2), false);
}
