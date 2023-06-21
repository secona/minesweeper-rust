use crate::util::colors::color_fg;
use termion::color;

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

    pub fn to_string_colored(&self) -> String {
        let value = self.to_string();

        match self {
            CellValue::Number(n) => match n {
                0 => String::from(" "), // transparent color
                1 => color_fg(value, color::LightBlue),
                2 => color_fg(value, color::Green),
                3 => color_fg(value, color::Red),
                4 => color_fg(value, color::Blue),
                5 => color_fg(value, color::Magenta),
                6 => color_fg(value, color::Cyan),
                7 => color_fg(value, color::LightCyan),
                8 => color_fg(value, color::White),
                _ => value,
            },
            CellValue::Bomb => color_fg(value, color::LightYellow),
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
        let number_cell_value = CellValue::Number(3);
        let bomb_cell_value = CellValue::Bomb;

        assert_eq!(number_cell_value.to_string(), "3");
        assert_eq!(bomb_cell_value.to_string(), "X");
    }

    #[test]
    fn increment_if_number_works() {
        let mut number_cell_value = CellValue::Number(3);
        let mut bomb_cell_value = CellValue::Bomb;

        number_cell_value.increment_if_number(1);
        if let CellValue::Number(num) = number_cell_value {
            assert_eq!(num, 4);
        }

        bomb_cell_value.increment_if_number(1);
    }
}
