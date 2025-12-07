use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use day07::Map;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Split,
    Empty,
}

fn process_input(input: &str) -> (Map<Tile>, (i32, i32)) {
    let map = Map::from_input(input, |x| match x {
        'S' => Some(Tile::Start),
        '^' => Some(Tile::Split),
        '.' => Some(Tile::Empty),
        _ => None,
    });

    let start = map
        .iter()
        .find_map(|(x, y, tile)| {
            if tile == &Tile::Start {
                Some((x, y))
            } else {
                None
            }
        })
        .unwrap();

    (map, start)
}

fn solve_a(input: &str) -> i64 {
    let (map, start) = process_input(input);

    let mut rays_before = HashSet::new();
    let mut rays_after = HashSet::new();

    rays_before.insert(start);

    let mut splits = 0;
    'outer: loop {
        for (ray_x, ray_y) in rays_before.iter() {
            let ray_x = *ray_x;
            let ray_y = *ray_y;
            match map.get(ray_x, ray_y + 1) {
                Some(Tile::Split) => {
                    rays_after.insert((ray_x - 1, ray_y + 1));
                    rays_after.insert((ray_x + 1, ray_y + 1));
                    splits += 1;
                }
                Some(Tile::Empty) => {
                    rays_after.insert((ray_x, ray_y + 1));
                }
                Some(Tile::Start) => unreachable!("Extra start tile found"),
                None => break 'outer,
            }
        }

        rays_before.clear();
        std::mem::swap(&mut rays_before, &mut rays_after);
    }

    splits
}

fn possible_timelines(cache: &mut HashMap<(i32, i32), i64>, map: &Map<Tile>, ray_x: i32, ray_y: i32) -> i64 {
    if let Some(timelines) =  cache.get(&(ray_x, ray_y)) {
        return *timelines;
    }

    let res = match map.get(ray_x, ray_y + 1) {
        Some(Tile::Start) => unreachable!(),
        Some(Tile::Empty) => possible_timelines(cache, map, ray_x, ray_y + 1),
        Some(Tile::Split) => {
            possible_timelines(cache, map, ray_x - 1, ray_y + 1)
                + possible_timelines(cache, map, ray_x + 1, ray_y + 1)
        }
        None => 1,
    };

    cache.insert((ray_x, ray_y), res);
    res
}

fn solve_b(input: &str) -> i64 {
    let (map, start) = process_input(input);
    let mut cache = HashMap::new();

    possible_timelines(&mut cache, &map, start.0, start.1)
}
