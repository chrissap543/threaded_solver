mod solver;
mod threaded;

use std::time::Instant; 

fn main() {
    let board = solver::test_board();

    let mut avg = vec![0; 30]; 
    for _ in 0..10 {
        let times = threaded::solve(&board); 
        for i in 0..times.len() {
            avg[i+1] += times[i]; 
        }
        let now = Instant::now(); 
        let _ = solver::solve(&board, (0, 0)); 
        let elapsed = now.elapsed(); 
        avg[0] += elapsed.subsec_micros(); 
    }
    for i in 0..avg.len() {
        avg[i] /= 10; 
    }
    for (i, num) in avg.iter().enumerate() {
        println!("{} worker(s) took on average {:.2?} microseconds", i+1, num); 
    }
}

// fn print_board(board: &Vec<Vec<u8>>) {
//     for vec in board {
//         for num in vec {
//             print!("{}, ", num);
//         }
//         println!();
//     }
// }
