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

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            Some(self.data[y as usize][x as usize])
        } else {
            None
        }
    }

    fn clear(&mut self, x: i32, y: i32) {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            self.data[y as usize][x as usize] = false;
        }
    }
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

    for y in 0..(map.height as i32) {
        for x in 0..(map.width as i32) {
            if map.get(x, y) != Some(true) {
                continue;
            }

            let mut rolls = 0;
            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if (i != 0 || j != 0) && map.get(x+i, y+j) == Some(true) {
                        rolls += 1;
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
    let mut map = process_input(input);
    let mut total_removed = 0;

    let mut to_remove = vec![];

    loop {
        to_remove.clear();
        for y in 0..(map.height as i32) {
            for x in 0..(map.width as i32) {
                if map.get(x, y) != Some(true) {
                    continue;
                }

                let mut rolls = 0;
                for i in [-1, 0, 1] {
                    for j in [-1, 0, 1] {
                        if (i != 0 || j != 0) && map.get(x+i, y+j) == Some(true) {
                            rolls += 1;
                        }
                    }
                }

                if rolls < 4 {
                    to_remove.push((x, y))
                }
            }
        }

        if to_remove.len() == 0 {
            break;
        }

        total_removed += to_remove.len();
        for (x, y) in &to_remove {
            map.clear(*x, *y);
        }
    }

    total_removed as i64
}
