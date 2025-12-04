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

struct Map {
    data: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

fn process_input(input: &str) -> Map {
    let data: Vec<Vec<bool>> = input
        .split("\n")
        .map(|x| {
            x.trim()
                .chars()
                .map(|x| {
                    if x == '.' {
                        Some(false)
                    } else if x == '@' {
                        Some(true)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        })
        .filter(|x: &Vec<bool>| x.len() > 0)
        .collect();

    let width = data.len();
    let height = data[0].len();
    Map {
        data,
        width,
        height,
    }
}

fn solve_a(input: &str) -> i64 {
    let map = process_input(input);

    let mut count = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if !map.data[y][x] {
                continue;
            }

            let mut rolls = 0;
            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    let xx = x as i32 + i;
                    let yy = y as i32 + j;

                    if xx > 0 && xx < map.width as i32 && yy > 0 && yy < map.height as i32 && (i != 0 || j != 0) {
                        if map.data[xx as usize][yy as usize] {
                            rolls += 1
                        }
                    }
                }
            }

            if rolls < 4 {
                count += 1;
            }
        }
    }

    count
}

fn solve_b(input: &str) -> i64 {
    0
}
