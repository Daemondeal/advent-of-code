pub mod point;

use std::collections::HashSet;
use std::env;
use std::fs;

use point::Point;

struct Movement {
    direction: Point,
    amount: i32,
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

fn process_input(input: &str) -> Vec<Movement> {
    input
        .split('\n')
        .flat_map(|line| {
            let mut tokens = line.split(' ');
            let direction_char = tokens.next()?;
            let amount: i32 = tokens.next()?.parse().unwrap();

            let direction = match direction_char {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, -1),
                "D" => (0, 1),
                _ => panic!("Unexpected direction."),
            };

            Some(Movement {
                direction: direction.into(),
                amount,
            })
        })
        .collect()
}

fn distance(head: Point, tail: Point) -> i32 {
    std::cmp::max((head.x - tail.x).abs(), (head.y - tail.y).abs())
}

fn solve_a(input: &str) -> i32 {
    let movements = process_input(input);
    let mut visited = HashSet::new();

    let mut head_position: Point = (0, 0).into();
    let mut tail_position: Point = (0, 0).into();

    visited.insert(tail_position);

    for Movement { direction, amount } in movements {
        for _ in 0..amount {
            let old_head = head_position;

            head_position = head_position + direction;
            if distance(head_position, tail_position) > 1 {
                tail_position = old_head;
                visited.insert(tail_position);
            }
        }
    }

    visited.len() as i32
}

fn solve_b(input: &str) -> i32 {
    let movements = process_input(input);
    let mut visited = HashSet::new();

    let mut snake: [Point; 10] = [(0, 0).into(); 10];

    visited.insert(snake[9]);

    for Movement { direction, amount } in movements {
        for _ in 0..amount {
            let mut new_snake = snake;
            new_snake[0] = snake[0] + direction;

            for (i, part) in snake.iter().enumerate().skip(1) {
                if distance(*part, new_snake[i - 1]) > 1 {
                    let delta_x = (new_snake[i - 1].x - part.x).clamp(-1, 1);
                    let delta_y = (new_snake[i - 1].y - part.y).clamp(-1, 1);

                    new_snake[i] = snake[i] + (delta_x, delta_y).into();
                }
            }

            snake = new_snake;

            visited.insert(snake[9]);
        }
    }

    visited.len() as i32
}

#[allow(dead_code)]
fn show_visited(visited: &HashSet<Point>) {
    let min_x = visited.iter().map(|p| p.x).min().unwrap();
    let min_y = visited.iter().map(|p| p.y).min().unwrap();
    let max_x = visited.iter().map(|p| p.x).max().unwrap();
    let max_y = visited.iter().map(|p| p.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x + 1 {
            if (x, y) == (0, 0) {
                print!("s");
            } else if visited.contains(&(x, y).into()) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
