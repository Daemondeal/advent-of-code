use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Write};
use std::fs;

use regex::Regex;

const RESOLVED_REGEX: &str = r"(\w+):\s(\d+)";
const UNRESOLVED_REGEX: &str = r"(\w+):\s(\w+)\s([+\-*/])\s(\w+)";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: cargo run -- [input_file]");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    println!("A: {}", solve_a(&input));
    println!("B: {:?}", solve_b(&input));
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => f.write_char('+'),
            Operation::Subtract => f.write_char('-'),
            Operation::Multiply => f.write_char('*'),
            Operation::Divide => f.write_char('/'),
            Operation::Equals => f.write_char('='),
        }
    }
}

impl Operation {
    fn calculate(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Subtract => lhs - rhs,
            Operation::Multiply => lhs * rhs,
            Operation::Divide => lhs / rhs,
            Operation::Equals => i64::from(lhs == rhs),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Target {
    Operation(Operation, Box<Target>, Box<Target>),
    Variable(String),
    Number(i64),
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Operation(op, lhs, rhs) => f.write_fmt(format_args!("({lhs} {op} {rhs})")),
            Target::Variable(name) => f.write_str(name),
            Target::Number(num) => f.write_fmt(format_args!("{num}")),
        }
    }
}

fn process_input(input: &str) -> HashMap<String, Target> {
    let resolved_re = Regex::new(RESOLVED_REGEX).unwrap();
    let unresolved_re = Regex::new(UNRESOLVED_REGEX).unwrap();

    let mut monkeys = HashMap::new();

    for captures in resolved_re.captures_iter(input.trim()) {
        monkeys.insert(
            captures[1].to_string(),
            Target::Number(captures[2].parse().unwrap()),
        );
    }

    for capture in unresolved_re.captures_iter(input.trim()) {
        let lhs = Target::Variable(capture[2].to_string());
        let rhs = Target::Variable(capture[4].to_string());
        monkeys.insert(
            capture[1].to_string(),
            Target::Operation(
                match &capture[3] {
                    "+" => Operation::Add,
                    "/" => Operation::Divide,
                    "*" => Operation::Multiply,
                    "-" => Operation::Subtract,
                    other => {
                        panic!("Unexpected operation {other}");
                    }
                },
                Box::new(lhs),
                Box::new(rhs),
            ),
        );
    }

    monkeys
}

fn merge(monkeys: &HashMap<String, Target>, target: &Target) -> Target {
    match target {
        Target::Number(n) => Target::Number(*n),
        Target::Variable(var) => {
            if let Some(monkey) = monkeys.get(var) {
                merge(monkeys, monkey)
            } else {
                Target::Variable(var.clone())
            }
        }
        Target::Operation(op, lhs, rhs) => Target::Operation(
            op.clone(),
            Box::new(merge(monkeys, lhs)),
            Box::new(merge(monkeys, rhs)),
        ),
    }
}

fn eval(target: &Target, human_value: i64) -> i64 {
    match target {
        Target::Operation(op, lhs, rhs) => {
            op.calculate(eval(lhs, human_value), eval(rhs, human_value))
        }
        Target::Variable(_) => human_value,
        Target::Number(num) => *num,
    }
}

fn simplify(target: &Target) -> Target {
    match target {
        Target::Operation(op, lhs, rhs) => {
            let lhs = if let Target::Operation(_, _, _) = lhs.borrow() {
                simplify(lhs)
            } else {
                *lhs.clone()
            };
            let rhs = if let Target::Operation(_, _, _) = rhs.borrow() {
                simplify(rhs)
            } else {
                *rhs.clone()
            };

            if let Target::Number(x) = lhs {
                if let Target::Number(y) = rhs {
                    return Target::Number(op.calculate(x, y));
                }
            }

            Target::Operation(op.clone(), Box::new(lhs), Box::new(rhs))
        }
        Target::Variable(var) => Target::Variable(var.clone()),
        Target::Number(num) => Target::Number(*num),
    }
}

fn solve_a(input: &str) -> i64 {
    let monkeys = process_input(input);

    let merged = merge(&monkeys, &monkeys["root"]);

    let simplified = simplify(&merged);

    if let Target::Number(result) = simplified {
        result
    } else {
        panic!("Invalid result: {simplified}")
    }
}

fn find_equality(target: &Target) -> (Target, i64) {
    match target {
        Target::Operation(op, lhs, rhs) => {
            if op == &Operation::Equals {
                match lhs.borrow() {
                    Target::Number(n) => (*rhs.to_owned(), *n),
                    _ => match rhs.borrow() {
                        Target::Number(n) => (*lhs.to_owned(), *n),
                        _ => panic!("Unexpected Operation"),
                    },
                }
            } else {
                find_equality(lhs)
            }
        }
        Target::Variable(_) => panic!("Unexpected Operation"),
        Target::Number(_) => panic!("Unexpected Operation"),
    }
}

fn solve_b(input: &str) -> Vec<i64> {
    let mut monkeys = process_input(input);
    monkeys.remove("humn");

    let new_root = if let Target::Operation(_, lhs, rhs) = &monkeys["root"] {
        Target::Operation(Operation::Equals, lhs.clone(), rhs.clone())
    } else {
        panic!("Invalid root");
    };

    monkeys.insert("root".to_string(), new_root);

    let merged = merge(&monkeys, &monkeys["root"]);

    let simplified = simplify(&merged);

    let (search, value_to_match) = find_equality(&simplified);

    // Secant Method
    let mut inputs = vec![1000, 2000];
    let mut outputs = vec![
        eval(&search, inputs[0]) - value_to_match,
        eval(&search, inputs[1]) - value_to_match,
    ];

    loop {
        let i = inputs.len() - 1;
        let next =
            inputs[i] - outputs[i] * (inputs[i] - inputs[i - 1]) / (outputs[i] - outputs[i - 1]);
        let value = eval(&search, next) - value_to_match;

        if value == 0 {
            let mut possible_results = vec![];
            for i in 0..10 {
                if eval(&search, next - i) == value_to_match {
                    possible_results.push(next - i);
                }
                if eval(&search, next + i + 1) == value_to_match {
                    possible_results.push(next + i);
                }
            }
            return possible_results;
        } else {
            inputs.push(next);
            outputs.push(value);
        }
    }
}
