#[derive(Debug)]
pub struct State {
    name: String
}

impl State {

    pub fn new(name: String) -> State {
        State {
            name
        }
    }

}
