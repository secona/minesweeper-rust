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
}
