use std::collections::HashMap;

use crate::{yaml::yaml_utils, tm::tape::TapeMovement};

use super::{state::State, tape::Tape, transition::{TMInpuit, TMOutput}};

#[derive(Debug)]
pub struct TM {
    states: Vec<State>,
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
            states: vec![],
            initial_state: None,
            final_states: vec![],
            alphabet: vec![],
            input_symbols: vec![],
            blank_symbol: None,
            tape: Tape::new(),
            transitions: HashMap::new()
        }
    }

    pub fn write_tape(input: Vec<String>) {

    }
    
    pub fn load_yaml_file(&mut self, filepath: String) {
        let data = yaml_utils::read_from_file(filepath);

        let states = data.get("states").unwrap().as_sequence().unwrap().clone();
        let initial_state = data.get("initial_state").unwrap().clone();
        let final_states = data.get("final_states").unwrap().as_sequence().unwrap().clone();
        let alphabet = data.get("alphabet").unwrap().as_sequence().unwrap().clone();
        let input_symbols = data.get("input_symbols").unwrap().as_sequence().unwrap().clone();
        let blank_symbol = data.get("blank_symbol").unwrap().clone();
        let transitions = data.get("transitions").unwrap().as_sequence().unwrap().clone();

        println!("{:?}", transitions);

        self.states = states.iter()
            .map(|state| { State::new(state.as_str().unwrap().to_string()) })
            .collect();

        self.initial_state = Some(State::new(initial_state.as_str().unwrap().to_string()));

        self.final_states = final_states.iter()
            .map(|state| { State::new(state.as_str().unwrap().to_string()) })
            .collect();

        self.alphabet = alphabet.iter()
            .map(|symbol| { symbol.as_str().unwrap().to_string() })
            .collect();

        self.input_symbols = input_symbols.iter()
            .map(|symbol| { symbol.as_str().unwrap().to_string() })
            .collect();

        self.blank_symbol = Some(blank_symbol.as_str().unwrap().to_string());

        self.transitions.clear();
        transitions.iter()
            .map(|transition| { (transition.get("input").unwrap().as_str().unwrap(), transition.get("output").unwrap().as_str().unwrap()) })
            .map(|(input, output)| {
                let input = input.split(" ").collect::<Vec<&str>>();
                let output = output.split(" ").collect::<Vec<&str>>();

                let state = State::new(input[0].to_string());
                let read_symbol = input[1].to_string();
                let input_symbol = input[2].to_string();

                let output_state = State::new(output[0].to_string());
                let tape_movement = TapeMovement::new(output[1]);

                (TMInpuit::new(state, read_symbol, input_symbol), TMOutput::new(output_state, tape_movement))
            })
            .for_each(|(input, output)| { self.transitions.insert(input, output); });
    }

}
