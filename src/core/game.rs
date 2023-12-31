use crate::components::cell_state::CellState;
use crate::components::cell_value::CellValue;
use crate::components::Board;
use crate::components::Point;
use crate::util::colors::color_bg;
use crate::util::colors::color_fg;
use std::io;
use std::io::Write;
use std::process;
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::Keys;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct Game {
    pub board: Board,
    pub cursor_coord: Point,
    stdin: Keys<io::Stdin>,
    stdout: RawTerminal<io::Stdout>,
}

impl Game {
    pub fn new(board: Board) -> Result<Game, &'static str> {
        let stdin = io::stdin().keys();
        let stdout = io::stdout().into_raw_mode().unwrap();

        Ok(Game {
            board,
            cursor_coord: Point { x: 0, y: 0 },
            stdin,
            stdout,
        })
    }

    pub fn play(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide,
            self.display()
        )
        .unwrap();
        self.stdout.flush().unwrap();

        loop {
            if self.has_won() {
                break;
            }

            let c = self.stdin.next().unwrap().unwrap();
            write!(self.stdout, "{}{}", cursor::Goto(1, 1), clear::AfterCursor).unwrap();

            match c {
                Key::Left => self.move_cursor(&Point { x: -1, y: 0 }),
                Key::Right => self.move_cursor(&Point { x: 1, y: 0 }),
                Key::Up => self.move_cursor(&Point { x: 0, y: -1 }),
                Key::Down => self.move_cursor(&Point { x: 0, y: 1 }),
                Key::Char(' ') => self.reveal_selected(),
                Key::Char('F') | Key::Char('f') => self.toggle_flag(),
                Key::Char('Q') | Key::Char('q') | Key::Ctrl('C') | Key::Ctrl('c') => break,
                _ => {}
            }

            write!(self.stdout, "{}", self.display()).unwrap();
            self.stdout.flush().unwrap();
        }

        loop {
            write!(
                self.stdout,
                "{}{}{}{}congratulations! press r to play again! press q to exit",
                cursor::Goto(1, 1),
                clear::AfterCursor,
                self.display(),
                cursor::Goto(1, (self.board.size + 1) as u16)
            )
            .unwrap();
            self.stdout.flush().unwrap();

            let c = self.stdin.next().unwrap().unwrap();

            match c {
                Key::Char('R') | Key::Char('r') => {
                    self.reset();
                    self.play();
                }
                Key::Char('Q') | Key::Char('q') | Key::Ctrl('C') | Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            }
        }

        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }

    fn game_over(&mut self) {
        write!(
            self.stdout,
            "{}{}game over! press r to restart! press q to exit!",
            self.display(),
            cursor::Goto(1, (self.board.size + 1) as u16),
        )
        .unwrap();
        self.stdout.flush().unwrap();

        loop {
            let c = self.stdin.next().unwrap().unwrap();
            match c {
                Key::Char('R') | Key::Char('r') => {
                    self.reset();
                    self.play();
                }
                Key::Char('Q') | Key::Char('q') | Key::Ctrl('C') | Key::Ctrl('c') => {
                    write!(self.stdout, "{}", cursor::Show).unwrap();
                    self.stdout.flush().unwrap();
                    process::exit(0);
                }
                _ => (),
            }
        }
    }

    fn move_cursor(&mut self, value: &Point<i32>) {
        let new_coord = self.cursor_coord.offset_and_limit(value, self.board.size);

        if let Some(point) = new_coord {
            self.cursor_coord = point;
        }
    }

    fn reveal(&mut self, point: Point) {
        if self.board.grid[point.y][point.x].state == CellState::Flagged {
            return;
        }

        if self.board.grid[point.y][point.x].value == CellValue::Bomb {
            self.game_over();
        }

        self.board.grid[point.y][point.x].state = CellState::Revealed;

        if self.board.grid[point.y][point.x].value == CellValue::Number(0) {
            for neighbor_coord in point.neighboring_points(self.board.size) {
                let cell = &self.board.grid[neighbor_coord.y][neighbor_coord.x];
                if CellState::Default == cell.state {
                    if CellValue::Bomb != cell.value {
                        self.reveal(neighbor_coord)
                    }
                }
            }
        }
    }

    fn reveal_selected(&mut self) {
        self.reveal(self.cursor_coord);
    }

    fn toggle_flag(&mut self) {
        let current_cell = &mut self.board.grid[self.cursor_coord.y][self.cursor_coord.x];

        current_cell.state = match current_cell.state {
            CellState::Default => CellState::Flagged,
            CellState::Flagged => CellState::Default,
            _ => current_cell.state,
        }
    }

    fn display(&self) -> String {
        let mut result: Vec<String> = vec![];

        for (j, row) in self.board.grid.iter().enumerate() {
            let mut row_result: Vec<String> = vec![];
            for (i, cell) in row.iter().enumerate() {
                let c: String = match cell.state {
                    CellState::Default => color_fg(String::from("?"), color::LightRed),
                    CellState::Revealed => cell.value.to_string_colored(),
                    CellState::Flagged => color_fg(String::from("F"), color::Rgb(128, 0, 128)),
                };

                row_result.push(if (Point { x: i, y: j }) == self.cursor_coord {
                    color_bg(c, color::LightBlack)
                } else {
                    c
                })
            }
            result.push(row_result.join(" "));
        }

        result.join("\r\n")
    }

    fn reset(&mut self) {
        self.board = Board::shuffle_bombs(&self.board);
        self.cursor_coord = Point { x: 0, y: 0 };
    }

    fn has_won(&self) -> bool {
        for row in self.board.grid.iter() {
            for cell in row.iter() {
                if let CellValue::Number(_) = cell.value {
                    if cell.state == CellState::Default {
                        return false;
                    }
                }
            }
        }

        true
    }
}
