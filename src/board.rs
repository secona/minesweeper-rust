use crate::Cell;
use crate::Point;
use rand::Rng;
use std::fmt::Display;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub size: usize,
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

        Board { cells, size }
    }

    pub fn populate(mut self, bomb_count: usize) -> Result<Board, &'static str> {
        let bomb_coords = self.place_bombs(bomb_count)?;
        self.increment_numbers_around_bombs(bomb_coords)?;

        Ok(self)
    }

    fn place_bombs(&mut self, bomb_count: usize) -> Result<Vec<Point>, &'static str> {
        if bomb_count > (self.size * self.size) {
            return Err("bomb_count exceeded cell count.");
        }

        let mut bomb_coords: Vec<Point> = vec![];

        loop {
            let mut rng = rand::thread_rng();
            let point = Point {
                x: rng.gen_range(0..self.size),
                y: rng.gen_range(0..self.size),
            };

            if !bomb_coords.contains(&point) {
                self.cells[point.x][point.y] = Cell::Bomb;
                bomb_coords.push(point);
            }

            if bomb_coords.len() >= bomb_count {
                break;
            }
        }

        Ok(bomb_coords)
    }

    fn increment_numbers_around_bombs(
        &mut self,
        bomb_coords: Vec<Point>,
    ) -> Result<(), &'static str> {
        for coord in bomb_coords {
            for i in -1..=1 {
                for j in -1..=1 {
                    let coord = coord
                        .offset(&Point { x: i, y: j })
                        .and_then(|offseted| offseted.limit(self.size));

                    if let Some(Point { x, y }) = coord {
                        let current_cell = &mut self.cells[x][y];
                        *current_cell = match current_cell {
                            Cell::Number(n) => Cell::Number(*n + 1),
                            Cell::Bomb => continue,
                        };
                    }
                }
            }
        }

        Ok(())
    }
}
