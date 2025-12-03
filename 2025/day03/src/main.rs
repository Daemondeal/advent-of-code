use std::env;
use std::fs;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

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

fn max_jolt(bank: &[u64], i: usize, n: usize, cur: u64) -> u64 {
    let mut cur_max: u64 = 0;
    if n == 0 {
        return cur;
    }

    let mut cur_digit_max: u64 = 0; 
    for j in i..bank.len() {
        if j+n <= bank.len() && bank[j] > cur_digit_max {
            cur_digit_max = bank[j];
            let tot = max_jolt(bank, j+1, n-1, cur*10+bank[j]);
            if tot > cur_max {
                cur_max = tot;
            }
        }
    }

    return cur_max;
}

fn max_joltage_12(bank: &[u64]) -> u64 {
    max_jolt(bank, 0, 12, 0)
}

fn solve_a(input: &str) -> i64 {
    process_input(input)
        .iter()
        .map(|x| max_joltage_2(x) as i64)
        .sum()
}

fn solve_b(input: &str) -> i64 {
    process_input(input)
        .par_iter()
        .map(|x| max_joltage_12(x) as i64)
        .sum()
}
