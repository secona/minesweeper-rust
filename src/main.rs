use minesweeper::Board;
use std::{
    io::{stdin, stdout, Write},
    process,
};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let board = Board::new(10);
    let board = board.populate(8).unwrap_or_else(|err| {
        eprintln!("Problem populating board: {err}");
        process::exit(1);
    });

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Hide,
        board
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Char(' ') => println!("Reveal"),
            Key::Char('F') | Key::Char('f') => println!("Flag"),
            Key::Char('Q') | Key::Char('q') | Key::Ctrl('C') | Key::Ctrl('c') => break,
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
