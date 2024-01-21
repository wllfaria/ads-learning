struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut next_gen: Vec<Vec<i32>> = vec![];
        for _ in 0..board.len() {
            next_gen.push(vec![]);
        }
        let directions: Vec<(i32, i32)> = vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
        ];

        for r in 0..board.len() {
            for c in 0..board[r].len() {
                let mut alive_neighbors = 0;
                for direction in directions.iter() {
                    if r as i32 + direction.0 < 0
                        || r as i32 + direction.0 >= board.len() as i32
                        || c as i32 + direction.1 < 0
                        || c as i32 + direction.1 >= board[r].len() as i32
                    {
                        continue;
                    }
                    let neighbor =
                        board[(r as i32 + direction.0) as usize][(c as i32 + direction.1) as usize];
                    if neighbor > 0 {
                        alive_neighbors += 1;
                    }
                }
                let cell = board[r][c];
                match cell {
                    x if x == 1 && alive_neighbors < 2 => next_gen[r].push(0),
                    x if x == 1 && alive_neighbors <= 3 => next_gen[r].push(1),
                    x if x == 1 && alive_neighbors > 3 => next_gen[r].push(0),
                    x if x == 0 && alive_neighbors == 3 => next_gen[r].push(1),
                    _ => next_gen[r].push(cell),
                }
            }
        }

        *board = next_gen;
    }
}

fn main() {
    let mut board_a = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    let mut board_b = vec![vec![1, 1], vec![1, 0]];

    Solution::game_of_life(&mut board_a);
    Solution::game_of_life(&mut board_b);

    assert_eq!(
        board_a,
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
    );
    assert_eq!(board_b, vec![[1, 1], [1, 1]]);
}
