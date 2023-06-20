use super::cell::CellValue;
use crate::components::Cell;
use crate::components::Point;
use rand::Rng;

pub struct Board {
    pub grid: Vec<Vec<Cell>>,
    pub size: usize,
    pub bomb_coords: Vec<Point>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let grid = vec![vec![Cell::default(); size]; size];

        Board {
            grid,
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
                self.grid[point.y][point.x].value = CellValue::Bomb;
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
                        self.grid[y][x].value.increment_if_number(1);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let board = Board::new(3);

        assert_eq!(board.bomb_coords, vec![]);
        assert_eq!(board.size, 3);
        assert_eq!(
            board.grid,
            vec![
                vec![Cell::default(), Cell::default(), Cell::default()],
                vec![Cell::default(), Cell::default(), Cell::default()],
                vec![Cell::default(), Cell::default(), Cell::default()]
            ]
        );
    }

    #[test]
    fn place_bombs_works() {
        let mut board = Board::new(3);
        let _ = board.place_bombs(3);
        for coord in board.bomb_coords {
            assert_eq!(board.grid[coord.y][coord.x], Cell::bomb());
        }
    }

    #[test]
    fn increment_numbers_around_bombs_works() {
        let mut board = Board {
            grid: vec![
                vec![Cell::bomb(), Cell::default(), Cell::default()],
                vec![Cell::default(), Cell::default(), Cell::default()],
                vec![Cell::default(), Cell::default(), Cell::bomb()],
            ],
            size: 3,
            bomb_coords: vec![Point { x: 0, y: 0 }, Point { x: 2, y: 2 }],
        };

        let _ = board.increment_numbers_around_bombs();
        assert_eq!(
            board.grid,
            vec![
                vec![Cell::bomb(), Cell::default(), Cell::default()],
                vec![Cell::default(), Cell::default(), Cell::default()],
                vec![Cell::default(), Cell::default(), Cell::bomb()],
            ],
        )
    }
}
