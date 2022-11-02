mod tm;
mod yaml;

use tm::turing_machine::TM;

fn main() {
    let mut turing_machine = TM::new();
    println!("{:?}", turing_machine);

    turing_machine.load_yaml_file(String::from("machine.yaml"));
    println!("{:?}", turing_machine);
}
