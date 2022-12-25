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
    println!("B: Merry Christmas!");
}

fn process_input(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.as_bytes()
                .iter()
                .flat_map(|c| match c {
                    b'1' => Some(1),
                    b'2' => Some(2),
                    b'0' => Some(0),
                    b'-' => Some(-1),
                    b'=' => Some(-2),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn evaluate_snafu(snafu: &Vec<i8>) -> i64 {
    let mut result = 0;
    for (i, digit) in snafu.iter().rev().enumerate() {
        result += (*digit as i64) * 5i64.pow(i as u32);
    }

    result
}

fn snafu_to_string(snafu: &Vec<i8>) -> String {
    snafu
        .iter()
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => ' ',
        })
        .collect()
}

fn to_base_5(input: i64) -> Vec<i8> {
    if input == 0 {
        return vec![0];
    }

    let base = 5i64;
    let mut value = input;
    let mut result = vec![];

    while value > 0 {
        result.push((value % base) as i8);
        value /= base;
    }

    result.into_iter().rev().collect()
}

fn to_snafu(input: i64) -> Vec<i8> {
    let base_5 = to_base_5(input);
    let mut snafu = vec![];

    let mut carry = 0;

    for digit in base_5.into_iter().rev() {
        let snafu_digit = match digit + carry {
            0 | 1 | 2 => {
                let d = digit + carry;
                carry = 0;
                d
            }
            3 => {
                carry = 1;
                -2
            }
            4 => {
                carry = 1;
                -1
            }
            5 => {
                carry = 1;
                0
            }
            6 => {
                carry = 1;
                1
            }
            d => panic!("Got invalid digit {d}"),
        };

        snafu.push(snafu_digit);
    }

    if carry > 0 {
        snafu.push(carry);
    }

    snafu.into_iter().rev().collect()
}

fn solve_a(input: &str) -> String {
    let snafu_numbers = process_input(input);

    let sum: i64 = snafu_numbers.iter().map(evaluate_snafu).sum();
    let snafu_sum = to_snafu(sum);

    assert_eq!(evaluate_snafu(&snafu_sum), sum);

    snafu_to_string(&snafu_sum)
}
