#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub state: CellState,
    pub value: CellValue,
}

impl Cell {
    pub fn default() -> Self {
        Self {
            state: CellState::Default,
            value: CellValue::Number(0),
        }
    }

    pub fn number(number: i32) -> Self {
        Self {
            state: CellState::Default,
            value: CellValue::Number(number),
        }
    }

    pub fn bomb() -> Self {
        Self {
            state: CellState::Default,
            value: CellValue::Bomb,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CellValue {
    Number(i32),
    Bomb,
}

impl CellValue {
    pub fn to_string(&self) -> String {
        match self {
            CellValue::Number(num) => num.to_string(),
            CellValue::Bomb => String::from("X"),
        }
    }

    pub fn increment_if_number(&mut self, value: i32) {
        if let CellValue::Number(num) = self {
            *num += value;
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CellState {
    Default,
    Revealed,
    Flagged,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_to_string_works() {
        let number_cell = Cell::number(3);
        let bomb_cell = Cell::bomb();

        assert_eq!(number_cell.value.to_string(), "3");
        assert_eq!(bomb_cell.value.to_string(), "X");
    }

    #[test]
    fn value_increment_if_number_works() {
        let mut number_cell = Cell::number(3);
        let mut bomb_cell = Cell::bomb();

        number_cell.value.increment_if_number(1);
        if let CellValue::Number(num) = number_cell.value {
            assert_eq!(num, 4);
        }

        bomb_cell.value.increment_if_number(1);
    }
}
