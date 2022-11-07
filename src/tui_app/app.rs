use crate::{tm::turing_machine::{TM, Halt}};

#[derive(Debug)]
enum MachineStep { Advance, Return }

pub struct App {
    turing_machine: TM,
    buffer: String
}

impl App {

    pub fn new() -> App {
        App {
            turing_machine: TM::new(),
            buffer: String::new()
        }
    }

    pub fn init(&mut self, filepath: String) {
        ncurses::initscr();
        ncurses::keypad(ncurses::stdscr(), true);

        self.turing_machine = TM::new();
        self.turing_machine.load_yaml_file(&filepath);
    }

    fn render_machine(&self) {
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

        let current_step = self.turing_machine.current_step;
        ncurses::addstr(format!("Passo atual: {}", current_step).as_str());
    }

    fn render_tape(&self) {
        ncurses::addstr("\n\n");
        
        let tape = self.turing_machine.tape.data.join(" ");

        ncurses::addstr(format!("{}\n", tape).as_str());
        let cursor = "  ".repeat(self.turing_machine.tape.position);
        ncurses::addstr(format!("{}*\n", cursor).as_str());
    }

    fn get_next_movement(&self) -> Option<MachineStep> {
        let input = ncurses::get_wch().unwrap();

        match input {
            ncurses::WchResult::KeyCode(ncurses::KEY_RIGHT) => Some(MachineStep::Advance),
            ncurses::WchResult::KeyCode(ncurses::KEY_LEFT) => Some(MachineStep::Return),
            _ => None
        }
    }

    fn get_tape_input(&mut self) -> Vec<String> {
        ncurses::getstr(&mut self.buffer);

        self.buffer
            .split(" ")
            .map(String::from)
            .collect()
    }

    fn accept(&self) {
        ncurses::addstr("ACEITA!\n");
    }

    fn reject(&self) {
        ncurses::addstr("REJEITA!\n");
    }

    pub fn run(&mut self) {
        ncurses::addstr("Digite o conteudo da fita: ");

        let input = self.get_tape_input();
        self.turing_machine.write_tape(input);

        loop {
            self.render_machine();
            self.render_tape();

            if let Some(movement) = self.get_next_movement() {
                if let Some(halt) = match movement {
                    MachineStep::Advance => self.turing_machine.step(),
                    MachineStep::Return  => {
                        self.turing_machine.rewind();
                        None
                    }
                } {
                    match halt {
                        Halt::Accept => self.accept(),
                        Halt::Reject => self.reject()
                    };

                    ncurses::getch();
                    break;
                }
            }
        }
    }
}

impl Drop for App {

    fn drop(&mut self) {
        ncurses::endwin();
    }

}
