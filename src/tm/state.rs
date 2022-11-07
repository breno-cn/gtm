#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State {
    pub name: String
}

impl State {

    pub fn new(name: String) -> State {
        State {
            name
        }
    }

}
