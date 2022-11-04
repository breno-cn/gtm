#[derive(Debug, Hash, PartialEq, Eq)]
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

    pub fn write_tape(&mut self, input: Vec<String>) {
        self.data = input;
    }

}
