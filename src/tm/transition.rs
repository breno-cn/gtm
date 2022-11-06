use super::{state::State, tape::TapeMovement};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TMInpuit {
    pub state: State,
    pub read_symbol: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct  TMOutput {
    pub state: State,
    pub input_symbol: String,
    pub tape_movement: TapeMovement
}

impl TMInpuit {

    pub fn new(state: State, read_symbol: String,) -> TMInpuit {
        TMInpuit { state, read_symbol }
    }

}

impl TMOutput {

    pub fn new(state: State, input_symbol: String, tape_movement: TapeMovement) -> TMOutput {
        TMOutput { state, input_symbol, tape_movement }
    }

}
