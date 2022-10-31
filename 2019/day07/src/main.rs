#![allow(dead_code)]

mod intcode;
use intcode::IntcodeMachine;

fn main() {
    let input = include_str!("input.txt");
    
    println!("A: {}", solve_1(input));
    println!("B: {}", solve_2(input));
}

fn solve_2(input: &str) -> i32 {


    best_chain_2(
        input, 
        &[5, 6, 7, 8, 9], 
        [false, false, false, false, false], 
        [0, 0, 0, 0, 0],
        0
    )


    // feedback_chain_tasks(&[9,8,7,6,5], input)
}

fn feedback_chain_tasks(inputs: &[i32;5], source: &str) -> i32 {
    let machine_template = IntcodeMachine::parse(source);

    let mut machines = vec![];

    for i in 0..inputs.len() {
        machines.push(machine_template.clone());
        machines[i].run_until_input();
        machines[i].restore_with_input(inputs[i]);
    }

    let mut current = 0;
    let mut last_halt = false;
    
    while !last_halt {
        for (i, machine) in machines.iter_mut().enumerate() {
            let (halted, output) = machine.restore_with_input(current);

            current = output[0];

            if i == inputs.len() - 1 {
                last_halt = halted;
            }
        }
    }


    current
}

fn best_chain_2(input: &str, values: &[i32; 5], mut mark: [bool; 5], mut so_far: [i32; 5], pos: usize) -> i32 {
    if pos == 5 {
        return feedback_chain_tasks(&so_far, input);
    }

    let mut max_val = 0;

    for i in 0..values.len() {
        if !mark[i] {
            mark[i] = true;
            so_far[pos] = values[i];

            let val = best_chain_2(input, values, mark, so_far, pos + 1);
            if val > max_val {
                max_val = val;
            }

            mark[i] = false;
        }
    }

    max_val
}


fn chain_tasks(inputs: &[i32;5], source: &str) -> i32 {
    let machine_template = IntcodeMachine::parse(source);
    let mut current = 0;

    for val in inputs {
        let mut machine = machine_template.clone();

        current = machine.run_collecting_outputs(&[*val, current])[0];
    }

    current
}

fn solve_1(input: &str) -> i32{
    best_chain_1(
        input, 
        &[0,1,2,3,4], 
        [false, false, false, false, false], 
        [0, 0, 0, 0, 0],
        0
    )
}

fn best_chain_1(input: &str, values: &[i32; 5], mut mark: [bool; 5], mut so_far: [i32; 5], pos: usize) -> i32 {
    if pos == 5 {
        return chain_tasks(&so_far, input);
    }

    let mut max_val = 0;

    for i in 0..values.len() {
        if !mark[i] {
            mark[i] = true;
            so_far[pos] = values[i];

            let val = best_chain_1(input, values, mark, so_far, pos + 1);
            if val > max_val {
                max_val = val;
            }

            mark[i] = false;
        }
    }

    max_val
}
