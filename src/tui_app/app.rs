use std::{borrow::Borrow, fmt::format};

use cursive::{CursiveRunnable, views::{TextView, NamedView, EditView}, view::Nameable};

use crate::{tm::turing_machine::TM};

pub struct App {
    turing_machine: TM
}

impl App {

    pub fn new() -> App {
        App {
            turing_machine: TM::new(),
        }
    }

    pub fn init(&mut self, filepath: String) {
        ncurses::initscr();
        ncurses::raw();
        ncurses::noecho();

        self.turing_machine = TM::new();
        self.turing_machine.load_yaml_file(&filepath);
    }

    pub fn render_machine(&self) {
        ncurses::clear();

        let states = self.turing_machine.states
            .iter()
            .map(|state| state.name.clone())
            .collect::<Vec<String>>()
            .join(" ");
        ncurses::addstr(format!("Estados: {}\n", states).as_str());

        let initial_state = self.turing_machine.initial_state.as_ref().unwrap().name.clone();
        ncurses::addstr(format!("Estado inicial: {}\n", initial_state).as_str());

        let final_states = self.turing_machine.final_states
            .iter()
            .map(|state| state.name.clone())
            .collect::<Vec<String>>()
            .join(" ");
        ncurses::addstr(format!("Estados finais: {}\n", final_states).as_str());

        let current_state = self.turing_machine.current_state.as_ref().unwrap().name.clone();
        ncurses::addstr(format!("Estado atual: {}\n", current_state).as_str());

        let alphabet = self.turing_machine.alphabet.join(" ");
        ncurses::addstr(format!("Alfabeto: {}\n", alphabet).as_str());

        let input_symbols = self.turing_machine.input_symbols.join(" ");
        ncurses::addstr(format!("Símbolos de entrada: {}\n", input_symbols).as_str());

        let blank_symbol = self.turing_machine.blank_symbol.as_ref().unwrap();
        ncurses::addstr(format!("Símbolo branco: {}\n", blank_symbol).as_str());

        let tape = self.turing_machine.tape.data.join(" ");
        ncurses::addstr(format!("Fita: {}\n", tape).as_str());

        let transitions = self.turing_machine.transitions
            .iter()
            .map(|(key, value)| {
                let input = format!("({}, {})", key.state.name, key.read_symbol);
                let output = format!("({}, {}, {})", value.state.name, value.input_symbol, value.tape_movement);

                format!("{} -> {}", input, output)
            })
            .collect::<Vec<String>>()
            .join("\n");
        ncurses::addstr(format!("{}\n", transitions).as_str());

        ncurses::getch();
    }

    pub fn run(&mut self) {
        loop {
            self.render_machine();
            break;
        }
    }

}

impl Drop for App {

    fn drop(&mut self) {
        ncurses::endwin();
    }

}
