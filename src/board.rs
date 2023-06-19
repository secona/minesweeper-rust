use crate::Cell;
use rand::Rng;
use std::fmt::Display;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub size: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: Vec<String> = vec![];
        for row in self.cells.clone().into_iter() {
            let mut line: Vec<String> = vec![];
            for cell in row {
                let value = match cell {
                    Cell::Number(num) => num.to_string(),
                    Cell::Bomb => String::from("X"),
                };
                line.push(value);
            }
            result.push(line.join(" "));
        }

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

    fn place_bombs(&mut self, bomb_count: usize) -> Result<Vec<(usize, usize)>, &'static str> {
        if bomb_count > (self.size * self.size) {
            return Err("bomb_count exceeded cell count.");
        }

        let mut bomb_coords: Vec<(usize, usize)> = vec![];

        for _ in 0..bomb_count {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.size);
            let y = rng.gen_range(0..self.size);
            self.cells[x][y] = Cell::Bomb;
            bomb_coords.push((x, y));
        }

        Ok(bomb_coords)
    }

    fn increment_numbers_around_bombs(
        &mut self,
        bomb_coords: Vec<(usize, usize)>,
    ) -> Result<(), &'static str> {
        for coord in bomb_coords {
            let x: i32 = coord.0.try_into().unwrap();
            let y: i32 = coord.1.try_into().unwrap();

            for i in -1..=1 {
                for j in -1..=1 {
                    let x_to_change: usize = match x + i {
                        d if d < 0 => continue,
                        d if d >= self.size.try_into().unwrap() => continue,
                        _ => (x + i).try_into().unwrap(),
                    };

                    let y_to_change: usize = match y + j {
                        d if d < 0 => continue,
                        d if d >= self.size.try_into().unwrap() => continue,
                        _ => (y + j).try_into().unwrap(),
                    };

                    let current_cell = &mut self.cells[x_to_change][y_to_change];
                    *current_cell = match current_cell {
                        Cell::Number(n) => Cell::Number(*n + 1),
                        Cell::Bomb => continue,
                    };
                }
            }
        }

        Ok(())
    }
}
