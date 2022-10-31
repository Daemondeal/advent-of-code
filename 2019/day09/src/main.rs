use intcode::IntcodeMachine;

mod intcode;


fn main() {
    let input = include_str!("input.txt");

    let mut machine = IntcodeMachine::parse(input);

    machine.run_manual();
}
