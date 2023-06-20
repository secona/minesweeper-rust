#[derive(Clone, PartialEq, Debug)]
pub enum CellState {
    Default,
    Revealed,
    Flagged,
}
