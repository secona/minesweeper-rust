use crate::components::Board;
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new(board: Board) -> Result<Game, &'static str> {
        Ok(Game { board })
    }

    pub fn play(&self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide,
            self.board
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
}
