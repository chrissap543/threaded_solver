use rand::Rng; 
use std::thread;  
use std::time::Instant; 

use crate::solver;

pub fn solve(board: &Vec<Vec<u8>>) -> Vec<u32> {
    let mut ans = vec![]; 
    let max_workers = 30;
    for workers in 2..max_workers+1 {
        let now = Instant::now(); 

        let mut worker_vect = vec![];     
        for _ in 0..workers {
            let row: usize = rand::thread_rng().gen_range(0..9); 
            let col: usize = rand::thread_rng().gen_range(0..9); 

            let board_copy = board.clone(); 
            let t = thread::spawn(move || solver::solve(&board_copy, (row, col))); 
            worker_vect.push(t); 
        }
        while unsafe { !solver::FINISHED } {} 

        let elapsed = now.elapsed(); 
        ans.push(elapsed.subsec_micros()); 
        for worker in worker_vect {
            worker.join().unwrap(); 
        }
        unsafe { solver::FINISHED = false; }
    }
    ans
}
