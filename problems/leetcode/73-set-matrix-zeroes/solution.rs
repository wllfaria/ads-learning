struct Solution {}

impl Solution {
    pub fn zero_row(matrix: &mut Vec<Vec<i32>>, p: (usize, usize)) {
        for i in 0..matrix[p.0].len() {
            matrix[p.0][i] = 0;
        }
    }

    pub fn zero_col(matrix: &mut Vec<Vec<i32>>, p: (usize, usize)) {
        for i in 0..matrix.len() {
            matrix[i][p.1] = 0;
        }
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut positions = Vec::new();
        for r in 0..matrix.len() {
            for c in 0..matrix[r].len() {
                match matrix[r][c] {
                    0 => positions.push((r, c)),
                    _ => (),
                }
            }
        }

        for p in &positions {
            Solution::zero_row(matrix, *p);
            Solution::zero_col(matrix, *p);
        }
    }
}

fn main() {
    let mut matrix_a = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let mut matrix_b = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    let output_a = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    let output_b = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
    Solution::set_zeroes(&mut matrix_a);
    Solution::set_zeroes(&mut matrix_b);
    assert_eq!(matrix_a, output_a);
    assert_eq!(matrix_b, output_b);
}
