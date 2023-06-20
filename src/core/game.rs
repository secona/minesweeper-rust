use crate::components::cell_state::CellState;
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
        let mut result: Vec<String> = vec![];

        for (j, row) in self.board.grid.iter().enumerate() {
            let mut row_result: Vec<String> = vec![];
            for (i, cell) in row.iter().enumerate() {
                let c: String = match cell.state {
                    CellState::Default => String::from("?"),
                    CellState::Revealed => cell.value.to_string(),
                    CellState::Flagged => String::from("F"),
                };

                row_result.push(if (Point { x: i, y: j }) == self.cursor_coord {
                    format!(
                        "{}{}{}",
                        color::Bg(color::Yellow),
                        c,
                        color::Bg(color::Reset)
                    )
                } else {
                    c
                })
            }
            result.push(row_result.join(" "));
        }

        write!(f, "{}", result.join("\r\n"))
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
                Key::Up => self.move_cursor(&Point { x: 0, y: -1 }),
                Key::Down => self.move_cursor(&Point { x: 0, y: 1 }),
                Key::Char(' ') => self.edit_on_cursor(CellState::Revealed),
                Key::Char('F') | Key::Char('f') => self.edit_on_cursor(CellState::Flagged),
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

    fn edit_on_cursor(&mut self, state: CellState) {
        let Point { x, y } = self.cursor_coord;
        self.board.grid[y][x].state = state;
    }
}
