use super::{state::State, tape::TapeMovement};

#[derive(Debug)]
pub struct TMInpuit {
    state: State,
    read_symbol: String,
    input_symbol: String
}

#[derive(Debug)]
pub struct  TMOutput {
    state: State,
    tape_movement: TapeMovement
}
