use rand::Rng;
use std::fmt::Display;

#[derive(Clone)]
pub enum Cell {
    Number(i32),
    Bomb,
}

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
        self.place_bombs(bomb_count)?;
        self.increment_numbers_around_bombs()?;

        Ok(self)
    }

    fn place_bombs(&mut self, bomb_count: usize) -> Result<(), &'static str> {
        if bomb_count > (self.size * self.size) {
            return Err("bomb_count exceeded cell count.");
        }

        for _ in 0..bomb_count {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.size);
            let y = rng.gen_range(0..self.size);
            self.cells[x][y] = Cell::Bomb;
        }

        Ok(())
    }

    fn increment_numbers_around_bombs(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
}
