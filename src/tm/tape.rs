use std::ops::Deref;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TapeMovement { L, R }

#[derive(Debug)]
pub struct Tape {
    data: Vec<String>,
    position: usize,
    tape_size: usize
}

impl TapeMovement {

    pub fn new(movement: &str) -> TapeMovement {
        match movement {
            "L" => TapeMovement::L,
            "R" => TapeMovement::R,
            _   => panic!("Erro na leitura do movimento, {:?}", movement)
        }
    }

}

impl Tape {

    pub fn new() -> Tape {
        let tape_size = 10;
        let initial_tape_pos = tape_size / 2;

        Tape {
            data: vec![String::from(""); tape_size],
            position: initial_tape_pos,
            tape_size
        }
    }

    pub fn reset_tape(&mut self) {
        self.position = self.tape_size / 2;
    }

    pub fn write_tape(&mut self, input: Vec<String>, blank_symbol: &String) {
        self.data.iter_mut()
            .for_each(|symbol| { *symbol = "".to_string() });

        self.position = 0;
        self.data[self.position] = blank_symbol.clone();
        self.position += 1;

        let input_length = input.len();
        for i in self.position..input_length+1 {
            self.data[i] = input[i - 1].clone();
        }

        self.position += input_length;
        self.data[self.position] = blank_symbol.clone();
        self.position = 1;
    }

    pub fn read(&mut self) -> String {
        self.data[self.position].clone()
    }

    pub fn write(&mut self, symbol: String) {
        self.data[self.position] = symbol;
    }

    pub fn move_tape(&mut self, movement: TapeMovement) {
        match movement {
            TapeMovement::L => self.position -= 1,
            TapeMovement::R => self.position += 1
        }
    }

}
