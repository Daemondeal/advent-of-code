use std::env;
use std::fs;

use itertools::Itertools;

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

fn process_input(input: &str) -> (Vec<i64>, Vec<(i64, i64, Vec<i64>)>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let tiles = &parts[0..parts.len() - 1];
    let reqs = parts[parts.len() - 1];

    let res_tiles = tiles
        .iter()
        .map(|x| x.chars().filter(|c| *c == '#').count() as i64)
        .collect();

    let res_reqs = reqs.split("\n").flat_map(|r| {
        let (dims_str, parts_numbers) = r.split(": ").collect_tuple()?;
        let (w, h): (i64, i64) = dims_str.split("x").flat_map(|x| x.parse::<i64>()).collect_tuple()?;
        let parts: Vec<i64> = parts_numbers.split(" ").flat_map(|x| x.parse()).collect();

        Some((w, h, parts))
    }).collect();

    (res_tiles, res_reqs)
}

fn fits(width: i64, height: i64, tiles: &[i64], amounts: &[i64]) -> bool {
    let total_pips: i64 = amounts.iter().zip(tiles.iter()).map(|(a, t)| (*t) * (*a)).sum();
    if total_pips > width * height {
        return false;
    }

    let space = amounts.iter().sum::<i64>() * 9;
    if space < width * height {
        return true;
    }


    // Unsure, assume true
    true
}

fn solve_a(input: &str) -> i64 {
    let (tiles, reqs) = process_input(input);

    reqs.iter().filter(|(w, h, am)| fits(*w, *h, &tiles, &am)).count() as i64
}

fn solve_b(input: &str) -> i64 {
    0
}
