#[derive(Clone, Eq, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        let number_cell = CellValue::Number(3);
        let bomb_cell = CellValue::Bomb;

        assert_eq!(number_cell.to_string(), "3");
        assert_eq!(bomb_cell.to_string(), "X");
    }

    #[test]
    fn increment_if_number_works() {
        let mut number_cell = CellValue::Number(3);
        let mut bomb_cell = CellValue::Bomb;

        number_cell.increment_if_number(1);
        if let CellValue::Number(num) = number_cell {
            assert_eq!(num, 4);
        }

        bomb_cell.increment_if_number(1);
    }
}
