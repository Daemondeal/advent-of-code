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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Multiplication,
}

fn process_row(input: &str) -> Vec<i64> {
    input
        .split(" ")
        .filter(|x| x.len() != 0)
        .map(|x| x.parse())
        .flatten()
        .collect()
}

fn process_input(input: &str) -> (Vec<Vec<i64>>, Vec<Operation>) {
    let rows: Vec<&str> = input.split("\n").filter(|x| x.len() > 0).collect();

    let numbers = rows[0..rows.len() - 1]
        .iter()
        .map(|x| process_row(x))
        .filter(|x| x.len() != 0)
        .collect();

    let ops = rows
        .last()
        .unwrap()
        .split(" ")
        .filter(|x| x.len() != 0)
        .map(|x| {
            if x.trim() == "*" {
                Operation::Multiplication
            } else {
                Operation::Addition
            }
        })
        .collect();

    (numbers, ops)
}

fn solve_a(input: &str) -> i64 {
    let (numbers, ops) = process_input(input);

    assert!(numbers[0].len() == ops.len());

    let mut count = 0;
    for i in 0..numbers[0].len() {
        let mut nums = vec![];
        for j in 0..numbers.len() {
            nums.push(numbers[j][i]);
        }

        let res = match ops[i] {
            Operation::Addition => nums.iter().sum::<i64>(),
            Operation::Multiplication => nums.into_iter().reduce(|acc, e| acc * e).unwrap(),
        };

        count += res;
    }

    count
}

fn solve_b(input: &str) -> i64 {
    let rows: Vec<Vec<char>> = input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .filter(|x| x.len() != 0)
        .collect();

    let mut count = 0;
    let mut operation = None;
    let mut numbers = vec![];
    for j in 0..rows[0].len() {
        let mut cur_number = vec![];

        for i in 0..rows.len() {
            if rows[i][j] != ' ' {
                cur_number.push(rows[i][j]);
            }
        }

        if operation.is_none() {
            operation = match cur_number.pop().unwrap() {
                '*' => Some(Operation::Multiplication),
                '+' => Some(Operation::Addition),
                _ => unreachable!("Expecting operation in first row"),
            }
        }

        if cur_number.len() == 0 {
            count += match operation.unwrap() {
                Operation::Addition => numbers.iter().sum::<i64>(),
                Operation::Multiplication => numbers.clone().into_iter().reduce(|acc, e| acc * e).unwrap(),
            };
            numbers.clear();
            operation = None;
        } else {
            numbers.push(cur_number.into_iter().collect::<String>().parse::<i64>().unwrap());
        }
    }

    count += match operation.unwrap() {
        Operation::Addition => numbers.iter().sum::<i64>(),
        Operation::Multiplication => numbers.into_iter().reduce(|acc, e| acc * e).unwrap(),
    };

    count
}
