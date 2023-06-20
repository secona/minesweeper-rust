use crate::components::CellValue;
use crate::components::Point;
use rand::Rng;

pub struct Board {
    pub cells: Vec<Vec<CellValue>>,
    pub size: usize,
    pub bomb_coords: Vec<Point>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let cells = vec![vec![CellValue::Number(0); size]; size];

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
                self.cells[point.y][point.x] = CellValue::Bomb;
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
                        self.cells[y][x].increment_if_number(1);
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
            board.cells,
            vec![
                vec![
                    CellValue::Number(0),
                    CellValue::Number(0),
                    CellValue::Number(0)
                ],
                vec![
                    CellValue::Number(0),
                    CellValue::Number(0),
                    CellValue::Number(0)
                ],
                vec![
                    CellValue::Number(0),
                    CellValue::Number(0),
                    CellValue::Number(0)
                ]
            ]
        );
    }

    #[test]
    fn place_bombs_works() {
        let mut board = Board::new(3);
        let _ = board.place_bombs(3);
        for coord in board.bomb_coords {
            assert_eq!(board.cells[coord.y][coord.x], CellValue::Bomb);
        }
    }

    #[test]
    fn increment_numbers_around_bombs_works() {
        let mut board = Board {
            cells: vec![
                vec![CellValue::Bomb, CellValue::Number(0), CellValue::Number(0)],
                vec![
                    CellValue::Number(0),
                    CellValue::Number(0),
                    CellValue::Number(0),
                ],
                vec![CellValue::Number(0), CellValue::Number(0), CellValue::Bomb],
            ],
            size: 3,
            bomb_coords: vec![Point { x: 0, y: 0 }, Point { x: 2, y: 2 }],
        };

        let _ = board.increment_numbers_around_bombs();
        assert_eq!(
            board.cells,
            vec![
                vec![CellValue::Bomb, CellValue::Number(1), CellValue::Number(0)],
                vec![
                    CellValue::Number(1),
                    CellValue::Number(2),
                    CellValue::Number(1)
                ],
                vec![CellValue::Number(0), CellValue::Number(1), CellValue::Bomb],
            ],
        )
    }
}
