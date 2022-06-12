pub fn test_board() -> Vec<Vec<u8>> {
    vec![
        vec![1, 7, 4, 0, 9, 0, 6, 0, 0],
        vec![0, 0, 0, 0, 3, 8, 1, 5, 7],
        vec![5, 3, 0, 7, 0, 1, 0, 0, 4],
        vec![0, 0, 7, 3, 4, 9, 8, 0, 0],
        vec![8, 4, 0, 5, 0, 0, 3, 6, 0],
        vec![3, 0, 5, 0, 0, 6, 4, 7, 0],
        vec![2, 8, 6, 9, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 6, 2, 7, 0, 3, 8],
        vec![0, 5, 3, 0, 8, 0, 0, 9, 6],
    ]
}

// todo implement any size board
pub fn valid(board: &Vec<Vec<u8>>, row: usize, column: usize, guess: u8) -> bool {
    valid_box(board, row, column, guess) & valid_rowcol(board, row, column, guess)
}
pub fn valid_box(board: &Vec<Vec<u8>>, row: usize, column: usize, guess: u8) -> bool {
    let x_index: usize = row / 3 * 3;
    let y_index: usize = column / 3 * 3;

    for x in 0..3 {
        for y in 0..3 {
            if board[x_index + x][y_index + y] == guess {
                return false;
            }
        }
    }
    true
}
pub fn valid_rowcol(board: &Vec<Vec<u8>>, row: usize, column: usize, guess: u8) -> bool {
    for x in 0..9 {
        if board[row][x] == guess || board[x][column] == guess {
            return false;
        }
    }
    true
}

pub fn next_empty(board: &Vec<Vec<u8>>) -> (usize, usize) {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                return (row, col);
            }
        }
    }
    (10, 10)
}
pub fn is_solved(board: &Vec<Vec<u8>>) -> bool {
    next_empty(board) == (10, 10)
}

pub fn guesses(board: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
    let mut res = vec![];
    for guess in 1..10 {
        if valid(board, row, col, guess) {
            res.push(guess);
        }
    }
    res
}

pub fn solve(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut answer = board.to_vec();
    let empty_cell = next_empty(&board);

    if is_solved(&board) {
        return answer;
    }

    let row = empty_cell.0;
    let col = empty_cell.1;

    for guess in guesses(board, row, col) {
        answer[row][col] = guess;
        answer = solve(&answer);
        if is_solved(&answer) {
            return answer;
        }
    }
    answer = board.to_vec();
    answer
}
