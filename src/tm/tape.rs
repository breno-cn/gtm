#[derive(Debug)]
pub enum TapeMovement { L, R }

#[derive(Debug)]
pub struct Tape {
    data: Vec<String>,
    position: usize
}

impl Tape {

    pub fn new() -> Tape {
        let tape_size = 10;
        let initial_tape_pos = tape_size / 2;

        Tape {
            data: vec![String::from(""); tape_size],
            position: initial_tape_pos
        }
    }

}
