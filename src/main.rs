use minesweeper::components::Board;
use minesweeper::core::Game;
use std::process;

fn main() {
    let board = Board::new(10).populate(8).unwrap_or_else(|err| {
        eprintln!("Problem populating board: {err}");
        process::exit(1);
    });

    let mut game = Game::new(board).unwrap_or_else(|err| {
        eprintln!("Problem initiating game: {err}");
        process::exit(1);
    });

    game.play();
}
