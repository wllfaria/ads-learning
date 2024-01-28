struct Solution {}

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();

        let n_spaces = text.chars().filter(|&c| c == ' ').count();

        let mut join = String::new();
        let mut pad = " ".repeat(n_spaces);
        if words.len() > 1 {
            join = " ".repeat(n_spaces / (words.len() - 1));
            pad = " ".repeat(n_spaces % (words.len() - 1));
        }

        words.join(&join) + &pad
    }
}

fn main() {
    let text_a = "  this   is  a sentence ";
    let out_a = "this   is   a   sentence";
    let text_b = " practice   makes   perfect";
    let out_b = "practice   makes   perfect ";

    assert_eq!(Solution::reorder_spaces(text_a.into()), out_a);
    assert_eq!(Solution::reorder_spaces(text_b.into()), out_b);
}
