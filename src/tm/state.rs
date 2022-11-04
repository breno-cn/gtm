#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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
