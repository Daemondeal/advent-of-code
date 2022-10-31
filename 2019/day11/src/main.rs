use crate::intcode::IntcodeMachine;

mod intcode;

fn main() {
    let input = include_str!("input.txt");
    solve_a(input);
}

fn solve_a(input: &str) -> i32 {

    let mut machine = IntcodeMachine::parse(input);

    machine.run_manual();
    123
}
