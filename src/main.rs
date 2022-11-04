mod tm;
mod yaml;

use tm::turing_machine::TM;

fn main() {
    let mut turing_machine = TM::new();
    println!("{:?}", turing_machine);

    turing_machine.load_yaml_file(String::from("machine.yaml"));
    println!("{:?}", turing_machine);

    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let words = buffer.clone().to_string();
    let input = words
        .trim()
        .split(" ")
        .map(|word| { word.to_string() })
        .collect::<Vec<String>>();

    println!("{:?}", input);

    turing_machine.write_tape(input);
    println!("{:?}", turing_machine);
}
