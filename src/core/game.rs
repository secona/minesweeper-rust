use crate::components::Board;
use crate::components::Point;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Game {
    pub board: Board,
    pub cursor_coord: Point,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row: Vec<String> = vec![];

        for i in 0..self.board.cells.len() {
            let mut col: Vec<String> = vec![];
            for j in 0..self.board.cells[i].len() {
                let value = self.board.cells[i][j].to_string();
                col.push(if self.cursor_coord == (Point { x: i, y: j }) {
                    format!(
                        "{}{}{}",
                        color::Bg(color::Yellow),
                        value,
                        color::Bg(color::Reset)
                    )
                } else {
                    value
                });
            }
            row.push(col.join(" "));
        }

        write!(f, "{}", row.join("\r\n"))
    }
}

impl Game {
    pub fn new(board: Board) -> Result<Game, &'static str> {
        Ok(Game {
            board,
            cursor_coord: Point { x: 0, y: 0 },
        })
    }

    pub fn play(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide,
            self
        )
        .unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            write!(stdout, "{}{}", cursor::Goto(1, 1), clear::AfterCursor).unwrap();

            match c.unwrap() {
                Key::Left => self.move_cursor(&Point { x: -1, y: 0 }),
                Key::Right => self.move_cursor(&Point { x: 1, y: 0 }),
                Key::Up => self.move_cursor(&Point { x: 0, y: 1 }),
                Key::Down => self.move_cursor(&Point { x: 0, y: -1 }),
                Key::Char(' ') => println!("Reveal"),
                Key::Char('F') | Key::Char('f') => println!("Flag"),
                Key::Char('Q') | Key::Char('q') | Key::Ctrl('C') | Key::Ctrl('c') => break,
                _ => {}
            }

            write!(stdout, "{}", self).unwrap();
            stdout.flush().unwrap();
        }

        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }

    fn move_cursor(&mut self, value: &Point<i32>) {
        let new_coord = self
            .cursor_coord
            .offset(value)
            .and_then(|offseted| offseted.limit(self.board.size));

        if let Some(point) = new_coord {
            self.cursor_coord = point;
        }
    }
}
