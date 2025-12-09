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
struct Point {
    x: i64,
    y: i64,
}

fn process_input(input: &str) -> Vec<Point> {
    input
        .split("\n")
        .map(|x| {
            let pt: Vec<i64> = x.split(",").map(|y| y.parse()).flatten().collect();

            if pt.len() == 2 {
                Some(Point { x: pt[0], y: pt[1] })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn solve_a(input: &str) -> i64 {
    let points = process_input(input);

    let mut area_max = 0;
    for px in &points {
        for py in &points {
            let x_dist = (px.x - py.x).abs() + 1;
            let y_dist = (px.y - py.y).abs() + 1;
            let area = x_dist * y_dist;
            if area > area_max {
                area_max = area;
            }
        }
    }

    area_max
}

fn solve_b(input: &str) -> i64 {
    let points = process_input(input);
    0
}
