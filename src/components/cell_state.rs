#[derive(Clone, PartialEq, Debug)]
pub enum CellState {
    Default,
    Revealed,
    Flagged,
}

impl CellState {
    pub fn reveal(&mut self) {
        *self = CellState::Revealed;
    }
}
