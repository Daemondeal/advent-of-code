mod point;

use std::collections::HashSet;
use std::env;
use std::fs;

use point::Point;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    WindNorth,
    WindSouth,
    WindWest,
    WindEast,
    Wall,
    Empty,
}

fn process_input(input: &str) -> Vec<Vec<Tile>> {
    let mut map = vec![];

    let rows: Vec<_> = input.trim().split('\n').collect();

    for i in 1..(rows.len() - 1) {
        let row = &rows[i];
        let bytes = row.as_bytes();

        let mut tile_row = vec![];
        for j in 1..(bytes.len() - 1) {
            let cell = bytes[j];
            let tile = match cell {
                b'>' => Tile::WindEast,
                b'<' => Tile::WindWest,
                b'^' => Tile::WindNorth,
                b'v' => Tile::WindSouth,
                b'#' => Tile::Wall,
                _ => Tile::Empty,
            };

            tile_row.push(tile);
        }
        map.push(tile_row);
    }

    map
}

fn is_blocked(map: &[Vec<Tile>], position: Point, time: i64, width: usize, height: usize) -> bool {
    let start = (0, -1).into();
    let target = (width - 1, height).into();

    if position == target || position == start {
        false
    } else {
        if position.x < 0
            || position.y < 0
            || position.x >= width as i64
            || position.y >= height as i64
        {
            true
        } else {
            let x = position.x;
            let y = position.y;

            let ww = width as i64;
            let wh = height as i64;

            let t1 = map[(y + time).rem_euclid(wh) as usize][x as usize];
            let t2 = map[(y - time).rem_euclid(wh) as usize][x as usize];
            let t3 = map[y as usize][(x + time).rem_euclid(ww) as usize];
            let t4 = map[y as usize][(x - time).rem_euclid(ww) as usize];

            t1 == Tile::WindNorth
                || t2 == Tile::WindSouth
                || t3 == Tile::WindWest
                || t4 == Tile::WindEast
        }
    }
}

fn solve_a(input: &str) -> i64 {
    let map = process_input(input);

    let start = (0i32, -1).into();
    let end = (map[0].len() - 1, map.len()).into();

    go_from_to(&map, 0, start, end)
}

fn go_from_to(map: &[Vec<Tile>], start_time: i64, from: Point, to: Point) -> i64 {
    let width = map[0].len();
    let height = map.len();

    let mut attempts: HashSet<(Point, i64)> = HashSet::new();
    attempts.insert((from, start_time));

    loop {
        let mut new_attempts = HashSet::new();
        for (position, time) in &attempts {
            for direction in [(0i32, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_position = *position + direction.into();
                if new_position == to {
                    return time + 1;
                }

                if !is_blocked(&map, new_position.clone(), time + 1, width, height) {
                    new_attempts.insert((new_position, time + 1));
                }
            }
        }

        attempts = new_attempts;
    }
}

fn solve_b(input: &str) -> i64 {
    let map = process_input(input);

    let start = (0i32, -1).into();
    let end = (map[0].len() - 1, map.len()).into();

    let t1 = go_from_to(&map, 0, start, end);
    let t2 = go_from_to(&map, t1, end, start);

    go_from_to(&map, t2, start, end)
}
