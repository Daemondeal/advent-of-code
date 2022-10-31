mod intcode;
use intcode::IntcodeMachine;

fn main() {
    let input = include_str!("input.txt");
    let mut machine = IntcodeMachine::parse(input);
    machine.run();
}