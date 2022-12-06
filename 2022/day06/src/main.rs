use std::collections::HashSet;
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

// fn process_input(input: &str) -> Vec<String> {
//     input.split("\n").map(|x| x.to_string()).collect()
// }

fn solve_a(input: &str) -> i32 {
    for (i, chunk) in input.as_bytes().windows(4).enumerate() {
        let uniques: HashSet<u8> = chunk.iter().map(|x| *x).collect();
        if uniques.len() == chunk.len() {
            return i as i32 + 4;
        }
    }

    -1
}

fn solve_b(input: &str) -> i32 {
    for (i, chunk) in input.as_bytes().windows(14).enumerate() {
        let uniques: HashSet<u8> = chunk.iter().map(|x| *x).collect();
        if uniques.len() == chunk.len() {
            return i as i32 + 14;
        }
    }

    -1
}
