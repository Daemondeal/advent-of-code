mod point;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

use point::Point;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Elf = usize;

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

fn process_input(input: &str) -> HashMap<Point, Elf> {
    let mut elves = HashMap::new();

    for (i, row) in input.trim().split('\n').enumerate() {
        for (j, cell) in row.bytes().enumerate() {
            if cell == b'#' {
                let position = (j, i).into();
                elves.insert(position, 4);
            }
        }
    }

    elves
}

fn get_direction_checks(direction: Direction) -> [Point; 3] {
    match direction {
        Direction::North => [(1i32, -1).into(), (0i32, -1).into(), (-1i32, -1).into()],
        Direction::South => [(1i32, 1).into(), (0i32, 1).into(), (-1i32, 1).into()],
        Direction::West => [(-1i32, 1).into(), (-1i32, 0).into(), (-1i32, -1).into()],
        Direction::East => [(1i32, 1).into(), (1i32, 0).into(), (1i32, -1).into()],
    }
}

#[allow(dead_code)]
fn print_map(elves: &HashMap<Point, Elf>) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);

    for position in elves.keys() {
        if position.x < min_x {
            min_x = position.x;
        }

        if position.x > max_x {
            max_x = position.x;
        }

        if position.y < min_y {
            min_y = position.y;
        }

        if position.y > max_y {
            max_y = position.y;
        }
    }

    let spacing = 2;

    min_x -= spacing;
    min_y -= spacing;

    max_x += spacing;
    max_y += spacing;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let position = (x, y).into();
            if elves.contains_key(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_a(input: &str) -> i64 {
    let mut elves = process_input(input);

    let mut decision_queue = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for _ in 0..10 {
        let mut second_pass = HashMap::new();
        let mut proposals_done = HashMap::new();

        for position in elves.keys() {
            let mut proposal = 4;
            let mut should_move = false;

            'outer: for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if elves.contains_key(&(*position + (i, j).into())) {
                        should_move = true;
                        break 'outer;
                    }
                }
            }

            if !should_move {
                second_pass.insert(*position, proposal);
                continue;
            }

            for (i, direction) in decision_queue.iter().enumerate() {
                let [side1, displacement, side2] = get_direction_checks(*direction);
                if !elves.contains_key(&(side1 + *position))
                    && !elves.contains_key(&(displacement + *position))
                    && !elves.contains_key(&(side2 + *position))
                {
                    proposal = i;
                    let new_position = *position + displacement;
                    if let Some(amount) = proposals_done.get_mut(&new_position) {
                        *amount += 1;
                    } else {
                        proposals_done.insert(new_position, 1);
                    }
                    break;
                }
            }
            second_pass.insert(*position, proposal);
        }

        elves.clear();

        for (position, elf_proposal) in second_pass {
            if elf_proposal == 4 {
                elves.insert(position, elf_proposal);
                continue;
            }

            let direction = decision_queue[elf_proposal];
            let movement = get_direction_checks(direction)[1];
            let new_position = movement + position;

            if proposals_done[&new_position] == 1 {
                elves.insert(new_position, elf_proposal);
            } else {
                elves.insert(position, elf_proposal);
            }
        }

        decision_queue.rotate_left(1);
    }

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);

    for position in elves.keys() {
        if position.x < min_x {
            min_x = position.x;
        }

        if position.x > max_x {
            max_x = position.x;
        }

        if position.y < min_y {
            min_y = position.y;
        }

        if position.y > max_y {
            max_y = position.y;
        }
    }

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let position = (x, y).into();
            if !elves.contains_key(&position) {
                count += 1;
            }
        }
    }

    count
}

fn solve_b(input: &str) -> i64 {
    let mut elves = process_input(input);

    let mut round = 0;
    let mut decision_queue = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    loop {
        round += 1;
        let mut second_pass = HashMap::new();
        let mut proposals_done = HashMap::new();

        let mut has_somebody_moved = false;

        for position in elves.keys() {
            let mut proposal = 4;
            let mut should_move = false;

            'outer: for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if elves.contains_key(&(*position + (i, j).into())) {
                        should_move = true;
                        break 'outer;
                    }
                }
            }

            if !should_move {
                second_pass.insert(*position, proposal);
                continue;
            }

            for (i, direction) in decision_queue.iter().enumerate() {
                let [side1, displacement, side2] = get_direction_checks(*direction);
                if !elves.contains_key(&(side1 + *position))
                    && !elves.contains_key(&(displacement + *position))
                    && !elves.contains_key(&(side2 + *position))
                {
                    proposal = i;
                    let new_position = *position + displacement;
                    if let Some(amount) = proposals_done.get_mut(&new_position) {
                        *amount += 1;
                    } else {
                        proposals_done.insert(new_position, 1);
                    }
                    break;
                }
            }
            second_pass.insert(*position, proposal);
        }

        elves.clear();

        for (position, elf_proposal) in second_pass {
            if elf_proposal == 4 {
                elves.insert(position, elf_proposal);
                continue;
            }

            let direction = decision_queue[elf_proposal];
            let movement = get_direction_checks(direction)[1];
            let new_position = movement + position;

            if proposals_done[&new_position] == 1 {
                elves.insert(new_position, elf_proposal);
                has_somebody_moved = true;
            } else {
                elves.insert(position, elf_proposal);
            }
        }

        if !has_somebody_moved {
            break;
        }

        decision_queue.rotate_left(1);
    }

    round
}
