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

#[derive(Debug)]
struct Range {
    lhs: i64,
    rhs: i64,
}

impl Range {
    fn contains(&self, val: i64) -> bool {
        return val <= self.rhs && val >= self.lhs;
    }
}


fn try_merge(head: &Range, candidate: &Range) -> Option<Range> {
    if candidate.lhs > head.rhs {
        None
    } else if candidate.rhs < head.rhs {
        Some(Range { lhs: head.lhs, rhs: head.rhs })
    } else {
        Some(Range { lhs: head.lhs, rhs: candidate.rhs })
    }
}

fn process_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges = parts[0]
        .split("\n")
        .map(|x| {
            let vals: Vec<i64> = x.trim().split("-").map(|x| x.parse::<i64>()).flatten().collect();

            if vals.len() == 2 {
                Some(Range { lhs: vals[0], rhs: vals[1] })
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let values = parts[1]
        .trim()
        .split("\n")
        .map(|x| x.parse::<i64>())
        .flatten()
        .collect();

    (ranges, values)
}

fn solve_a(input: &str) -> i64 {
    let (ranges, vals) = process_input(input);

    let mut count = 0;

    for val in &vals {
        for range in &ranges {
            if range.contains(*val) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn solve_b(input: &str) -> i64 {
    let (mut ranges, _) = process_input(input);

    ranges.sort_unstable_by(|x, y| y.lhs.cmp(&x.lhs));


    let mut merged_ranges = vec![];

    while !ranges.is_empty() {
        let mut head = ranges.pop().unwrap();

        loop {
            let Some(candidate) = ranges.pop() else {
                break;
            };

            let trial = try_merge(&head, &candidate);

            match trial {
                Some(range) => head = range,
                None => {
                    ranges.push(candidate);
                    break;
                }
            }
        }

        merged_ranges.push(head);
    }

    merged_ranges.iter().map(|x| x.rhs - x.lhs + 1).sum()
}
