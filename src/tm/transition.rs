use super::{state::State, tape::TapeMovement};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TMInpuit {
    state: State,
    read_symbol: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct  TMOutput {
    state: State,
    input_symbol: String,
    tape_movement: TapeMovement
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
