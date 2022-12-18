use super::{state::State, tape::TapeMovement};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TMInpuit {
    pub state: State,
    pub read_symbols: Vec<String>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct  TMOutput {
    pub state: State,
    pub input_symbols: Vec<String>,
    pub tape_movements: Vec<TapeMovement>
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Rewind {
    pub old_state: State,
    pub old_symbols: Vec<String>,
    pub rewind_movements: Vec<TapeMovement>
}

impl TMInpuit {

    pub fn new(state: State, read_symbols: Vec<String>) -> TMInpuit {
        TMInpuit { state, read_symbols }
    }

}

impl TMOutput {

    pub fn new(state: State, input_symbols: Vec<String>, tape_movements: Vec<TapeMovement>) -> TMOutput {
        TMOutput { state, input_symbols, tape_movements }
    }

}

impl Rewind {

    pub fn new(old_state: State, old_symbols: Vec<String>, rewind_movements: Vec<TapeMovement>) -> Rewind {
        Rewind { old_state, old_symbols, rewind_movements }
    }

}
