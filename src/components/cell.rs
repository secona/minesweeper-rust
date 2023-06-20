use super::{cell_state::CellState, cell_value::CellValue};

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub state: CellState,
    pub value: CellValue,
}

impl Cell {
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
