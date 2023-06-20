use crate::Cell;
use crate::Point;
use rand::Rng;
use std::fmt::Display;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub size: usize,
    pub bomb_coords: Vec<Point>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: Vec<String> = self
            .cells
            .iter()
            .map(|row| {
                let row: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
                row.join(" ")
            })
            .collect();

        write!(f, "{}", result.join("\n"))
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        let cells = vec![vec![Cell::Number(0); size]; size];

        Board {
            cells,
            size,
            bomb_coords: vec![],
        }
    }

    pub fn populate(mut self, bomb_count: usize) -> Result<Board, &'static str> {
        self.place_bombs(bomb_count)?;
        self.increment_numbers_around_bombs()?;

        Ok(self)
    }

    fn place_bombs(&mut self, bomb_count: usize) -> Result<(), &'static str> {
        if bomb_count > (self.size * self.size) {
            return Err("bomb_count exceeded cell count.");
        }

        loop {
            let mut rng = rand::thread_rng();
            let point = Point {
                x: rng.gen_range(0..self.size),
                y: rng.gen_range(0..self.size),
            };

            if !self.bomb_coords.contains(&point) {
                self.cells[point.x][point.y] = Cell::Bomb;
                self.bomb_coords.push(point);
            }

            if self.bomb_coords.len() >= bomb_count {
                break;
            }
        }

        Ok(())
    }

    fn increment_numbers_around_bombs(&mut self) -> Result<(), &'static str> {
        for coord in &self.bomb_coords {
            for i in -1..=1 {
                for j in -1..=1 {
                    let coord = coord
                        .offset(&Point { x: i, y: j })
                        .and_then(|offseted| offseted.limit(self.size));

                    if let Some(Point { x, y }) = coord {
                        self.cells[x][y].increment_if_number(1);
                    }
                }
            }
        }

        Ok(())
    }
}
