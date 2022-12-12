pub mod heap;

use std::collections::HashMap;
use std::env;
use std::fs;

use crate::heap::Heap;

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

fn process_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut result = vec![];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in input.split('\n').enumerate() {
        let mut row = vec![];
        for (j, char) in line.as_bytes().iter().enumerate() {
            row.push(match char {
                b'S' => {
                    start = (i, j);
                    0
                }
                b'E' => {
                    end = (i, j);
                    b'z' - b'a'
                }
                ch => ch - b'a',
            });
        }
        if !row.is_empty() {
            result.push(row);
        }
    }

    (result, start, end)
}

fn a_star_heuristic(a: (usize, usize), b: (usize, usize)) -> f32 {
    let x1 = a.0 as f32;
    let y1 = a.1 as f32;

    let x2 = b.0 as f32;
    let y2 = b.1 as f32;

    ((x1 - x2).abs() + (y1 - y2).abs()).sqrt()
}

fn get_neighbours(map: &[Vec<u8>], node: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let cell = map[node.0][node.1];
    for (i, j) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
        let x = (node.0 as i32) + i;
        let y = (node.1 as i32) + j;

        if x < 0 || y < 0 || x >= map.len() as i32 || y >= map[0].len() as i32 {
            continue;
        }

        let x = x as usize;
        let y = y as usize;

        if map[x][y] <= cell + 1 {
            res.push((x, y))
        }
    }

    res
}

fn get_length(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    end: (usize, usize),
    start: (usize, usize),
) -> i32 {
    let mut count = 0;
    let mut cur = end;

    loop {
        if cur == start {
            return count;
        }

        count += 1;
        cur = came_from[&cur];
    }
}

fn a_star(map: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    const INFINITY: f32 = f32::MAX;

    let mut open = Heap::new();

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();
    g_score.insert(start, 0.0);

    open.insert(start, 0.0);

    while let Some(current) = open.pop() {
        if current == end {
            return Some(get_length(came_from, end, start));
        }

        for neighbour in get_neighbours(map, current) {
            let g_test: f32 = g_score.get(&current).unwrap_or(&INFINITY) + 1f32;
            if g_test < *g_score.get(&neighbour).unwrap_or(&INFINITY) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, g_test);

                if !open.contains(&neighbour) {
                    open.insert(neighbour, g_test + a_star_heuristic(current, neighbour));
                }
            }
        }
    }

    None
}

fn solve_a(input: &str) -> i32 {
    let (map, start, end) = process_input(input);

    a_star(&map, start, end).unwrap()
}

fn solve_b(input: &str) -> i32 {
    let (map, _, end) = process_input(input);

    let mut best = i32::MAX;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                let Some(value) = a_star(&map, (i, j), end) else { continue; };
                if value < best {
                    best = value;
                }
            }
        }
    }

    best
}
