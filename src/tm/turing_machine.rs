use std::collections::HashMap;

use crate::{yaml::yaml_utils, tm::tape::{TapeMovement}};

use super::{state::State, tape::Tape, transition::{TMInpuit, TMOutput, Rewind}};

#[derive(Debug)]
pub enum Halt { Accept, Reject }

#[derive(Debug)]
pub struct TM {
    pub states: Vec<State>,
    pub initial_state: Option<State>,
    pub final_states: Vec<State>,
    pub current_state: Option<State>,
    pub alphabet: Vec<String>,
    pub input_symbols: Vec<String>,
    pub blank_symbol: Option<String>,
    pub tapes: Vec<Tape>,
    pub transitions: HashMap<TMInpuit, TMOutput>,
    pub rewinds: Vec<Rewind>,
    pub current_step: i32,
    pub n_tapes: i64
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
            tapes: vec![],
            transitions: HashMap::new(),
            rewinds: vec![],
            current_step: 0,
            n_tapes: 0
        }
    }

    pub fn write_tape(&mut self, inputs: Vec<Vec<String>>) {
        self.tapes.iter_mut()
            .zip(inputs)
            .for_each(|(tape, input)| tape.write_tape(input, self.blank_symbol.as_ref().unwrap()));
    }

    pub fn step(&mut self) -> Option<Halt> {
        let current_symbols = self.tapes.iter()
            .map(|tape| tape.read())
            .collect::<Vec<String>>();

        let current_state = self.current_state.clone().unwrap();
        let input = TMInpuit::new(current_state.clone(), current_symbols.clone());
        

        let output = self.transitions.get(&input);
        match output {
            Some(output) => {
                let state = output.state.clone();
                let input_symbols = output.input_symbols.clone();
                let tape_movements = output.tape_movements.clone();
    
                let reverse_movements  = tape_movements.iter()
                    .map(TapeMovement::reverse)
                    .collect::<Vec<TapeMovement>>();

                let rewind = Rewind::new(current_state, current_symbols, reverse_movements);
                self.rewinds.push(rewind);

                self.current_state = Some(state);

                self.tapes.iter_mut()
                    .zip(input_symbols)
                    .for_each(|(tape, input_symbol)| tape.write(input_symbol));

                self.tapes.iter_mut()
                    .zip(tape_movements)
                    .for_each(|(tape, tape_movement)| tape.move_tape(tape_movement));

                self.current_step += 1;

                None
            },
            None => Some(self.accept())
        }
    }

    pub fn accept(&self) -> Halt {
        if self.final_states.contains(&self.current_state.as_ref().unwrap()) {
            Halt::Accept
        } else {
            Halt::Reject
        }
    }

    pub fn rewind(&mut self) {
        if self.rewinds.is_empty() {
            return;
        }

        let rewind = self.rewinds.pop().unwrap();
        let old_state = rewind.old_state;
        let old_symbols = rewind.old_symbols;
        let rewind_movements = rewind.rewind_movements;

        self.current_state = Some(old_state);

        self.tapes.iter_mut()
            .zip(rewind_movements)
            .for_each(|(tape, rewind_movement)| tape.move_tape(rewind_movement));

        self.tapes.iter_mut()
            .zip(old_symbols)
            .for_each(|(tape, old_symbol)| tape.write(old_symbol));

        self.current_step -= 1;
    }
    
    pub fn load_yaml_file(&mut self, filepath: &String) {
        let data = yaml_utils::read_from_file(filepath);
        
        let states = data.get("states").unwrap().as_sequence().unwrap().clone();
        let initial_state = data.get("initial_state").unwrap().clone();
        let final_states = data.get("final_states").unwrap().as_sequence().unwrap().clone();
        let alphabet = data.get("alphabet").unwrap().as_sequence().unwrap().clone();
        let input_symbols = data.get("input_symbols").unwrap().as_sequence().unwrap().clone();
        let blank_symbol = data.get("blank_symbol").unwrap().clone();
        let n_tapes = data.get("n_tapes").unwrap().as_i64().unwrap().clone();
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

        self.n_tapes = n_tapes;
        self.tapes = vec![Tape::new(); n_tapes as usize];

        self.transitions.clear();
        transitions.iter()
            .map(|transition| {
                let input_state = transition.get("input_state").unwrap().as_str().unwrap().to_string();
                let input_state = State::new(input_state);
                let input_symbols = transition.get("input_symbols").unwrap()
                    .as_str()
                    .unwrap()
                    .split(" ")
                    .map(String::from)
                    .collect::<Vec<String>>();
                let input = TMInpuit::new(input_state, input_symbols);

                let output_state = transition.get("output_state").unwrap().as_str().unwrap().to_string();
                let output_state = State::new(output_state);
                let output_symbols = transition.get("output_symbols").unwrap()
                    .as_str()
                    .unwrap()
                    .split(" ")
                    .map(String::from)
                    .collect::<Vec<String>>();
                let tape_movements = transition.get("tape_movements").unwrap()
                    .as_str()
                    .unwrap()
                    .split(" ")
                    .map(TapeMovement::new)
                    .collect::<Vec<TapeMovement>>();
                let output = TMOutput::new(output_state, output_symbols, tape_movements);

                (input, output)
            })
            .for_each(|(input, output)| { self.transitions.insert(input, output); });
    }

}
