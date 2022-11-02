#[derive(Debug)]
pub struct TM {
    state: String
}

impl TM {

    pub fn new() -> TM {
        TM {
            state: String::from("test")
        }
    }

}
