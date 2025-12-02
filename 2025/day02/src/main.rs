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

fn process_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|x| {
            let z: Vec<i64> = x.trim().split("-").map(|y| y.parse::<i64>()).flatten().collect();
            if z.len() == 2 {
                Some((z[0], z[1]))
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn clog10(x: i64) -> i64 {
    (x as f64).log10().ceil() as i64
}

fn solve_a(input: &str) -> i64 {
    let ranges = process_input(input);

    let mut count: i64 = 0;
    for (l, r) in ranges {
        for i in l..=r {
            let len = clog10(i) as u32;

            if len % 2 == 0 {
                let base = i % (10_i64.pow(len/2));

                if ((base * 10_i64.pow(len/2)) + base) == i {
                    count += i as i64;
                }
            }

        }
    }

    count
}

fn solve_b(input: &str) -> i64 {
    let ranges = process_input(input);

    let mut count: i64 = 0;
    for (l, r) in ranges {
        for i in l..=r {
            let len = clog10(i) as u32;

            for j in 2..=len {
                if len % j == 0 {
                    let expon = 10_i64.pow(len/j);

                    let base = i % expon;

                    let mut acc = 0;
                    for _ in 0..j {
                        acc *= expon;
                        acc += base;
                    }
                    if acc == i {
                        count += i as i64;
                        break;
                    }
                }
            }
        }
    }

    count
}
