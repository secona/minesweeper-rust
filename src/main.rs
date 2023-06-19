use std::process;

use minesweeper::Board;

fn main() {
    let board = Board::new(10);
    let board = board.populate(8).unwrap_or_else(|err| {
        eprintln!("Problem populating board: {err}");
        process::exit(1);
    });
    println!("{board}");
}
