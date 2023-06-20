#[derive(Clone, Eq, Debug, PartialEq)]
pub enum Cell {
    Number(i32),
    Bomb,
}

impl Cell {
    pub fn to_string(&self) -> String {
        match self {
            Cell::Number(num) => num.to_string(),
            Cell::Bomb => String::from("X"),
        }
    }

    pub fn increment_if_number(&mut self, value: i32) {
        if let Cell::Number(num) = self {
            *num += value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        let number_cell = Cell::Number(3);
        let bomb_cell = Cell::Bomb;

        assert_eq!(number_cell.to_string(), "3");
        assert_eq!(bomb_cell.to_string(), "X");
    }

    #[test]
    fn increment_if_number_works() {
        let mut number_cell = Cell::Number(3);
        let mut bomb_cell = Cell::Bomb;

        number_cell.increment_if_number(1);
        if let Cell::Number(num) = number_cell {
            assert_eq!(num, 4);
        }

        bomb_cell.increment_if_number(1);
    }
}
