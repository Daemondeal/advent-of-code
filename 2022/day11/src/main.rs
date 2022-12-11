use std::env;
use std::fs;

use regex::Regex;

const MONKEY_REGEX: &str = r#"Monkey (\d+):\n\s*Starting items: (.*)\n\s*Operation: new = (.*)\n\s*Test: divisible by (\d+)\n\s*If true: throw to monkey (\d+)\n\s*If false: throw to monkey (\d)"#;

#[derive(Debug)]
enum Target {
    Number(i64),
    Old,
}

#[derive(Debug)]
enum Operation {
    Add(Target, Target),
    Multiply(Target, Target),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisibility: i64,
    throw_true: i64,
    throw_false: i64,
}

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

fn parse_operation(op: &str) -> Operation {
    let mut tokens = op.split(' ');

    let lhs = match tokens.next().unwrap() {
        "old" => Target::Old,
        num => Target::Number(num.parse().unwrap()),
    };

    let operation = tokens.next().unwrap();

    let rhs = match tokens.next().unwrap() {
        "old" => Target::Old,
        num => Target::Number(num.parse().unwrap()),
    };

    match operation {
        "*" => Operation::Multiply(lhs, rhs),
        "+" => Operation::Add(lhs, rhs),
        op => panic!("Unhandled operation {op}."),
    }
}

fn process_input(input: &str) -> Vec<Monkey> {
    let input_re = Regex::new(MONKEY_REGEX).unwrap();
    let mut result = vec![];

    for captures in input_re.captures_iter(input) {
        let items = captures[2].split(", ").flat_map(|x| x.parse()).collect();
        let divisibility = captures[4].parse().unwrap();
        let throw_true = captures[5].parse().unwrap();
        let throw_false = captures[6].parse().unwrap();
        let operation = parse_operation(&captures[3]);

        result.push(Monkey {
            items,
            operation,
            divisibility,
            throw_true,
            throw_false,
        })
    }

    result
}

fn evaluate_target(item: i64, target: &Target) -> i64 {
    match target {
        Target::Number(n) => *n,
        Target::Old => item,
    }
}

fn new_worry(item: i64, operation: &Operation, modulo: i64) -> i64 {
    match operation {
        Operation::Add(lhs, rhs) => {
            (evaluate_target(item, lhs) + evaluate_target(item, rhs)) % modulo
        }
        Operation::Multiply(lhs, rhs) => {
            (evaluate_target(item, lhs) * evaluate_target(item, rhs)) % modulo
        }
    }
}

fn solve_a(input: &str) -> i64 {
    let mut monkeys = process_input(input);
    let mut monkey_activity = vec![];
    for _ in &monkeys {
        monkey_activity.push(0);
    }

    let modulo = monkeys.iter().map(|m| m.divisibility).fold(1, |a, b| a * b);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let item = new_worry(monkeys[i].items[j], &monkeys[i].operation, modulo) / 3;
                let divisibility = monkeys[i].divisibility;
                if item % divisibility == 0 {
                    let throw_true = monkeys[i].throw_true as usize;
                    monkeys[throw_true].items.push(item);
                } else {
                    let throw_false = monkeys[i].throw_false as usize;
                    monkeys[throw_false].items.push(item);
                }

                monkey_activity[i] += 1;
            }
            monkeys[i].items.clear();
        }
    }

    monkey_activity.sort_by(|a, b| b.cmp(a));

    monkey_activity[0] * monkey_activity[1]
}

fn solve_b(input: &str) -> i64 {
    let mut monkeys = process_input(input);
    let mut monkey_activity = vec![];
    for _ in &monkeys {
        monkey_activity.push(0);
    }
    let modulo = monkeys.iter().map(|m| m.divisibility).fold(1, |a, b| a * b);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let item = new_worry(monkeys[i].items[j], &monkeys[i].operation, modulo);
                let divisibility = monkeys[i].divisibility;
                if item % divisibility == 0 {
                    let throw_true = monkeys[i].throw_true as usize;
                    monkeys[throw_true].items.push(item);
                } else {
                    let throw_false = monkeys[i].throw_false as usize;
                    monkeys[throw_false].items.push(item);
                }

                monkey_activity[i] += 1;
            }
            monkeys[i].items.clear();
        }
    }

    monkey_activity.sort_by(|a, b| b.cmp(a));

    monkey_activity[0] * monkey_activity[1]
}
