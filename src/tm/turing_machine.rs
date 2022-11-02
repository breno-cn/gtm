use std::collections::HashMap;

use super::{state::State, tape::Tape, transition::{TMInpuit, TMOutput}};

#[derive(Debug)]
pub struct TM {
    initial_state: Option<State>,
    final_states: Vec<State>,
    alphabet: Vec<String>,
    input_symbols: Vec<String>,
    blank_symbol: Option<String>,
    tape: Tape,
    transitions: HashMap<TMInpuit, TMOutput>
}

impl TM {

    pub fn new() -> TM {
        TM {
            initial_state: None,
            final_states: vec![],
            alphabet: vec![],
            input_symbols: vec![],
            blank_symbol: None,
            tape: Tape::new(),
            transitions: HashMap::new()
        }
    }

}
