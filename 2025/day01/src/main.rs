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

fn process_input(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .map(|x| {
            let dir = x.replace("L", "-").replace("R", "");
            dir.parse::<i32>()
        })
        .flatten()
        .collect()
}

fn solve_a(input: &str) -> i32 {
    let nums = process_input(input);

    let mut pos: i64 = 50;
    let mut count: i32 = 0;
    for num in nums {
        pos += num as i64;

        pos += 100 * 500;
        pos = pos % 100;
        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn solve_b(input: &str) -> i32 {
    let nums = process_input(input);

    let mut pos: i64 = 50;
    let mut count: i64 = 0;
    for num in nums {
        if num > 0 {
            for _ in 0..num {
                pos += 1;
                if pos >= 100 { pos -= 100 }
                if pos == 0 {
                    count += 1;
                }
            }
        } else {
            for _ in 0..(-num) {
                pos -= 1;
                if pos < 0 { pos += 100 }
                if pos == 0 {
                    count += 1;
                }
            }
        }
    }

    count as i32
}
