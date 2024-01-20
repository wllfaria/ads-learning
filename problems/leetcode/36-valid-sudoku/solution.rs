struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut rows: [HashSet<char>; 9] = Default::default();
        let mut cols: [HashSet<char>; 9] = Default::default();
        let mut boxs: [HashSet<char>; 9] = Default::default();

        for r in 0..board.len() {
            for c in 0..board.len() {
                if board[r][c] == '.' {
                    continue;
                }
                if rows[r].contains(&board[r][c]) {
                    return false;
                }
                if cols[c].contains(&board[r][c]) {
                    return false;
                }
                let box_idx = r / 3 * 3 + c / 3;
                if boxs[box_idx].contains(&board[r][c]) {
                    return false;
                }
                rows[r].insert(board[r][c]);
                cols[c].insert(board[r][c]);
                boxs[box_idx].insert(board[r][c]);
            }
        }

        true
    }
}

pub fn main() {
    let board_a = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let board_b = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let board_c = vec![
        vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
        vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
        vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
        vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    let board_d = vec![
        vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
        vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
        vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
        vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
        vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
        vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
    ];

    assert_eq!(Solution::is_valid_sudoku(board_a), true);
    assert_eq!(Solution::is_valid_sudoku(board_b), false);
    assert_eq!(Solution::is_valid_sudoku(board_c), false);
    assert_eq!(Solution::is_valid_sudoku(board_d), false);
}
