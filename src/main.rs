extern crate threadpool;

mod solver;
mod threaded;

fn main() {
    let board = solver::test_board();
    println!("Before board");
    print_board(&board);

    let sol = solver::solve(&board);
    println!("Solved board");
    print_board(&sol);
}

fn print_board(board: &Vec<Vec<u8>>) {
    for vec in board {
        for num in vec {
            print!("{}, ", num);
        }
        println!();
    }
}
