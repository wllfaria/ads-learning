struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

fn main() {
    let s_a = String::from("the sky is blue");
    let s_b = String::from("  hello world!  ");
    let s_c = String::from("a good   example");

    assert_eq!(
        Solution::reverse_words(s_a),
        String::from("blue is sky the")
    );
    assert_eq!(Solution::reverse_words(s_b), String::from("world! hello"));
    assert_eq!(Solution::reverse_words(s_c), String::from("example good a"));
}
