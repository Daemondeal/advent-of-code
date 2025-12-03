use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: cargo run -- [input_file]");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn process_input(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .split("\n")
        .map(|x| {
            x
                .trim()
                .chars()
                .map(|x| x.to_digit(10))
                .flatten()
                .map(|x| x as u64)
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn max_joltage_2(bank: &[u64]) -> u64 {
    let mut cur_max = 0;

    for i in 0..bank.len()-1 {
        if cur_max > (bank[i] * 10 + 9) {
            continue;
        }

        for j in (i+1)..bank.len() {
            let n = bank[i] * 10 + bank[j];
            if n > cur_max {
                cur_max = n;
            }
        }
    }


    cur_max
}

fn max_jolt(bank: &[u64], i: u64, n: u64) -> u64 {
    if (i+n) >= bank.len() as u64 {
        return 0;
    }


    1
}

fn max_joltage_12(bank: &[u64]) -> u64 {
    let mut cur_max = 0;

    for i in 0..bank.len()-1 {
        if cur_max > (bank[i] * 10 + 9) {
            continue;
        }

        for j in (i+1)..bank.len() {
            let n = bank[i] * 10 + bank[j];
            if n > cur_max {
                cur_max = n;
            }
        }
    }


    cur_max
}

fn solve_a(input: &str) -> i64 {
    process_input(input)
        .iter()
        .map(|x| max_joltage_2(x) as i64)
        .sum()
}

fn solve_b(input: &str) -> i64 {
    process_input(input)
        .iter()
        .map(|x| max_joltage_2(x) as i64)
        .sum()
}
