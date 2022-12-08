use std::collections::HashSet;
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

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split('\n')
        .map(|ch| {
            ch.chars()
                .flat_map(|n| n.to_string().parse::<i32>())
                .collect::<Vec<i32>>()
        })
        .filter(|row| !row.is_empty())
        .collect()
}

fn count_visible(
    map: &[Vec<i32>],
    visible: &mut HashSet<(i32, i32)>,
    start: (i32, i32),
    direction: (i32, i32),
) -> i32 {
    let mut max = -1;
    let mut count = 0;
    let mut position = start;

    while let Some(row) = map.get(position.0 as usize) {
        let Some(cell) = row.get(position.1 as usize) else { break };

        if cell > &max {
            if !visible.contains(&position) {
                count += 1;
                visible.insert(position);
            }
            max = *cell;
        }

        position = (position.0 + direction.0, position.1 + direction.1);
        if position.0 < 0 || position.1 < 0 {
            break;
        }
    }

    count
}

fn count_visible_from_tree(map: &[Vec<i32>], start: (i32, i32), direction: (i32, i32)) -> i32 {
    let tree = map[start.0 as usize][start.1 as usize];
    let mut count = 0;
    let mut position = start;

    position = (position.0 + direction.0, position.1 + direction.1);

    while let Some(row) = map.get(position.0 as usize) {
        let Some(cell) = row.get(position.1 as usize) else { break };
        count += 1;

        if *cell < tree {
        } else {
            break;
        }

        position = (position.0 + direction.0, position.1 + direction.1);
        if position.0 < 0 || position.1 < 0 {
            break;
        }
    }

    count
}

fn solve_a(input: &str) -> i32 {
    let map = process_input(input);
    let width = map.len() as i32;
    let height = map[0].len() as i32;

    let mut total = 0;

    let mut visible = HashSet::new();

    // Left and Right
    for i in 0..height {
        total += count_visible(&map, &mut visible, (0, i), (1, 0));
        total += count_visible(&map, &mut visible, (width - 1, i), (-1, 0));
    }

    // Top and Bottom
    for i in 0..width {
        total += count_visible(&map, &mut visible, (i, 0), (0, 1));
        total += count_visible(&map, &mut visible, (i, height - 1), (0, -1));
    }

    total
}

fn solve_b(input: &str) -> i32 {
    let map = process_input(input);
    let width = map.len() as i32;
    let height = map[0].len() as i32;

    let mut max_score = 0;

    for x in 0..width {
        for y in 0..height {
            let position = (x, y);
            let right = count_visible_from_tree(&map, position, (1, 0));
            let left = count_visible_from_tree(&map, position, (-1, 0));
            let bottom = count_visible_from_tree(&map, position, (0, 1));
            let top = count_visible_from_tree(&map, position, (0, -1));

            let score = right * left * bottom * top;

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}
