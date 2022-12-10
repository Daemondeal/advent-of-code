use std::env;
use std::fs;

enum Instruction {
    Addx(i32),
    Noop,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: cargo run -- [input_file]");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    println!("A: {}", solve_a(&input));
    println!("B: \n{}", solve_b(&input));
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .flat_map(|line| {
            let mut tokens = line.split(' ');
            let instruction = tokens.next()?;
            match instruction {
                "addx" => Some(Instruction::Addx(tokens.next()?.parse().unwrap())),
                "noop" => Some(Instruction::Noop),
                _ => None, //panic!("Unexpected Instruction {instruction}"),
            }
        })
        .collect()
}

fn solve_a(input: &str) -> i32 {
    let program = process_input(input);

    let mut signal_strength = 0;
    let mut x_register = 1;
    let mut cycle_count = 0;

    let notable_cycles = &[20, 60, 100, 140, 180, 220];

    for instruction in program {
        match instruction {
            Instruction::Addx(amount) => {
                cycle_count += 1;
                if notable_cycles.contains(&cycle_count) {
                    signal_strength += cycle_count * x_register;
                }
                cycle_count += 1;
                if notable_cycles.contains(&cycle_count) {
                    signal_strength += cycle_count * x_register;
                }
                x_register += amount;
            }
            Instruction::Noop => {
                cycle_count += 1;
                if notable_cycles.contains(&cycle_count) {
                    signal_strength += cycle_count * x_register;
                }
            }
        }
    }

    signal_strength
}

fn tick_crt(crt_string: &mut String, x_register: i32) {
    let line_position = crt_string.split('\n').last().unwrap().len();

    if (x_register - line_position as i32).abs() <= 1 {
        crt_string.push('#');
    } else {
        crt_string.push('.');
    }

    if line_position == 39 {
        crt_string.push('\n');
    }
}

fn solve_b(input: &str) -> String {
    let program = process_input(input);

    let mut x_register = 1;
    let mut crt = "".to_string();

    for instruction in program {
        match instruction {
            Instruction::Addx(amount) => {
                tick_crt(&mut crt, x_register);
                tick_crt(&mut crt, x_register);
                x_register += amount;
            }
            Instruction::Noop => {
                tick_crt(&mut crt, x_register);
            }
        }
    }

    crt
}
