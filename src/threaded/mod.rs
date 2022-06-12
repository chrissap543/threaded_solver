use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

use crate::solver;
use solver::*;

pub fn solve(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let workers = 8;
    let pool = ThreadPool::new(workers);

    let board = Arc::new(Mutex::new(board)); 
    
    pool.join(); 
}

fn execute(board: Arc<Vec<Vec<u8>>>, pool: ThreadPool) {
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
