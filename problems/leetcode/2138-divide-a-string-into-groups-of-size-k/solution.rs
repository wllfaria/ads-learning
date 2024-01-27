struct Solution {}

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut s = s;
        if s.len() % k as usize != 0 {
            s += &fill.to_string().repeat(k as usize - s.len() % k as usize);
        }
        let n: usize = s.len() / k as usize;
        let mut res: Vec<String> = Vec::with_capacity(n);
        for i in 0..n {
            let from = k as usize * i;
            let to = k as usize * (i + 1);
            res.push(s[from..to].to_string());
        }
        res
    }
}

fn main() {
    let s_a = "abcdefghi".to_string();
    let k = 3;
    let f = 'x';
    let s_b = "abcdefghij".to_string();

    assert_eq!(
        Solution::divide_string(s_a, k, f),
        vec!["abc", "def", "ghi"],
    );
    assert_eq!(
        Solution::divide_string(s_b, k, f),
        vec!["abc", "def", "ghi", "jxx"],
    )
}
