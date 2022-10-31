mod intcode;
use crate::intcode::IntcodeMachine;

fn main() {
    let input = include_str!("input.txt");
    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn solve_a(input: &str) -> i32 {
    let mut machine = IntcodeMachine::parse(input);

    machine.state[1] = 12;
    machine.state[2] = 2;

    machine.run();

    machine.state[0]
}

fn solve_b(input: &str) -> i32 {
    let root = IntcodeMachine::parse(input);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut machine = root.clone();

            machine.state[1] = noun;
            machine.state[2] = verb;

            machine.run();

            if machine.state[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Not found!");
}



