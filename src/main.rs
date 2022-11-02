mod tm;

use tm::turing_machine::TM;

fn main() {
    let turing_machine = TM::new();
    println!("{:?}", turing_machine);
}
