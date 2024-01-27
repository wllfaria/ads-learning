struct Solution {}

/*
 * NOTE:
 * This is a mess by now, it took me a couple of hours to figure this out
 * but I managed to get it right. I'll try to explain the approach below:
 * 1. First we iterate over the words and add the maximum amount of words
 * per index that we can.
 *      - this is done by summing up the lengths of the words + minimum spaces
 *      required
 * 2. Iterate over the lines and check how much spaces are needed
 *      - distribute evenly the spaces from left to right
 * 3. Just join the last line and append spaces to the right
 */
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ans = vec![String::new()];
        let mut total_len = 0;
        let mut idx = 0;
        for i in 0..words.len() {
            if total_len + words[i].len() <= max_width as usize {
                total_len += words[i].len() + 1;
                ans[idx].push_str(&words[i]);
                ans[idx].push_str(" ");
            } else {
                total_len = 0;
                ans.push(String::new());
                idx += 1;
                total_len += words[i].len() + 1;
                ans[idx].push_str(&words[i]);
                ans[idx].push_str(" ");
            }
        }
        let mut justified = ans
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .map(|z| z.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<_>>();

        for i in 0..justified.len() {
            let count = justified[i].len() as i32;
            let len = justified[i].iter().fold(0, |mut a, v| {
                a += v.len() as i32;
                a
            });
            let mut missing = max_width - len;
            let mut ix: i32 = 0;
            if count == 1 {
                justified[i][0].push_str(&" ".repeat(missing as usize));
                continue;
            }
            if i == justified.len() - 1 {
                let curr = &mut justified[i];
                let mut added: usize = 0;
                for j in 0..curr.len() {
                    if j == curr.len() - 1 {
                        curr[j].push_str(&" ".repeat(missing as usize - added));
                    } else {
                        added += 1;
                        curr[j].push_str(" ");
                    }
                }
                break;
            }
            while missing > 0 {
                missing -= 1;
                justified[i][ix as usize].push_str(" ");
                ix = (ix + 1) % (count - 1);
            }
        }
        let ret = justified
            .iter()
            .map(|x| x.join(""))
            .collect::<Vec<String>>();
        ret
    }
}

fn main() {
    let wo_a = vec![
        "This",
        "is",
        "an",
        "example",
        "of",
        "text",
        "justification.",
    ];
    let wi_a = 16;
    let wo_b = vec!["What", "must", "be", "acknowledgment", "shall", "be"];
    let wi_b = 16;
    let wo_c = vec![
        "Science",
        "is",
        "what",
        "we",
        "understand",
        "well",
        "enough",
        "to",
        "explain",
        "to",
        "a",
        "computer.",
        "Art",
        "is",
        "everything",
        "else",
        "we",
        "do",
    ];
    let wi_c = 20;

    assert_eq!(
        Solution::full_justify(wo_a.iter().map(|s| s.to_string()).collect(), wi_a),
        vec!["This    is    an", "example  of text", "justification.  ",]
    );
    assert_eq!(
        Solution::full_justify(wo_b.iter().map(|s| s.to_string()).collect(), wi_b),
        vec!["What   must   be", "acknowledgment  ", "shall be        ",]
    );
    assert_eq!(
        Solution::full_justify(wo_c.iter().map(|s| s.to_string()).collect(), wi_c),
        vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ]
    );
}
