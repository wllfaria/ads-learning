struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let len = usize::max(word1.len(), word2.len());
        let mut merged = String::with_capacity(len);
        for i in 0..len {
            if i < word1.len() {
                merged.push(word1.chars().nth(i).unwrap());
            }
            if i < word2.len() {
                merged.push(word2.chars().nth(i).unwrap());
            }
        }
        merged
    }
}

fn main() {
    let word_a_1 = "abc";
    let word_a_2 = "pqr";
    let word_b_1 = "ab";
    let word_b_2 = "pqrs";
    let word_c_1 = "abcd";
    let word_c_2 = "pq";

    assert_eq!(
        Solution::merge_alternately(word_a_1.to_string(), word_a_2.to_string()),
        "apbqcr"
    );
    assert_eq!(
        Solution::merge_alternately(word_b_1.to_string(), word_b_2.to_string()),
        "apbqrs"
    );
    assert_eq!(
        Solution::merge_alternately(word_c_1.to_string(), word_c_2.to_string()),
        "apbqcd"
    );
}
