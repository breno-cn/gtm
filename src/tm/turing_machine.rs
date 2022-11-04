use std::collections::HashMap;

use crate::{yaml::yaml_utils, tm::tape::TapeMovement};

use super::{state::State, tape::Tape, transition::{TMInpuit, TMOutput}};

#[derive(Debug)]
pub struct TM {
    states: Vec<State>,
    initial_state: Option<State>,
    final_states: Vec<State>,
    current_state: Option<State>,
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
            current_state: None,
            alphabet: vec![],
            input_symbols: vec![],
            blank_symbol: None,
            tape: Tape::new(),
            transitions: HashMap::new()
        }
    }

    pub fn write_tape(&mut self, input: Vec<String>) {
        self.tape.reset_tape();
        self.tape.write_tape(input);
    }

    pub fn step(&mut self) {
        // let input = TMInpuit::new(self.initial_state, read_symbol, input_symbol)
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

        self.current_state = self.initial_state.clone();

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

                let output_state = State::new(output[0].to_string());
                let input_symbol = output[1].to_string();
                let tape_movement = TapeMovement::new(output[2]);

                (TMInpuit::new(state, read_symbol), TMOutput::new(output_state, input_symbol,tape_movement))
            })
            .for_each(|(input, output)| { self.transitions.insert(input, output); });
    }

}
