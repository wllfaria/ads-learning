struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let is_vowel = |b: u8| {
            matches!(
                b,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        };
        let (mut lo, mut hi) = (0, s.len() - 1);
        let mut vb = s.clone().into_bytes();

        while lo < hi {
            while lo < hi && !is_vowel(vb[lo]) {
                lo += 1;
            }
            while lo < hi && !is_vowel(vb[hi]) {
                hi -= 1;
            }
            if lo < hi {
                vb.swap(lo, hi);
                lo += 1;
                hi -= 1;
            }
        }
        String::from_utf8(vb).unwrap()
    }
}

fn main() {
    let s_1 = "hello".to_string();
    let s_2 = "leetcode".to_string();
    let s_3 = "aA".to_string();
    assert_eq!(Solution::reverse_vowels(s_1), "holle".to_string());
    assert_eq!(Solution::reverse_vowels(s_2), "leotcede".to_string());
    assert_eq!(Solution::reverse_vowels(s_3), "Aa".to_string());
}
