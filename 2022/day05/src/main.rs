use std::env;
use std::fs;

use regex::Regex;

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

fn process_input(input: &str) -> (Vec<Vec<u8>>, Vec<(usize, usize, usize)>) {
    let mut line_iter = input.split('\n');
    let mut bins = vec![];

    for line in line_iter.by_ref() {
        if line.starts_with(" 1 ") {
            break;
        }

        for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
            if bins.len() <= i {
                bins.push(vec![]);
            }

            if chunk[1] != 32 {
                bins[i].insert(0, chunk[1]);
            }
        }
    }
    line_iter.next();

    let move_re = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    let mut moves = vec![];

    for line in line_iter {
        if let Some(captures) = move_re.captures(line) {
            moves.push((
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
                captures.get(3).unwrap().as_str().parse().unwrap(),
            ));
        }
    }

    (bins, moves)
}

fn solve_a(input: &str) -> String {
    let (mut bins, moves) = process_input(input);

    for (count, from, to) in moves {
        for _ in 0..count {
            let element = bins[from - 1].pop().unwrap();
            bins[to - 1].push(element);
        }
    }

    let output_bytes: Vec<u8> = bins.iter().map(|x| *x.last().unwrap()).collect();

    String::from_utf8(output_bytes).unwrap()
}

fn solve_b(input: &str) -> String {
    let (mut bins, moves) = process_input(input);

    let mut buffer = vec![];
    for (count, from, to) in moves {
        for _ in 0..count {
            buffer.push(bins[from - 1].pop().unwrap());
        }

        for element in buffer.iter().rev() {
            bins[to - 1].push(*element);
        }

        buffer.clear();
    }

    let output_bytes: Vec<u8> = bins.iter().map(|x| *x.last().unwrap()).collect();

    String::from_utf8(output_bytes).unwrap()
}
