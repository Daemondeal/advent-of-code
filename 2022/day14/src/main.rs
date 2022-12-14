pub mod point;

use std::env;
use std::fmt::{Display, Write};
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

struct Map {
    items: Vec<Vec<Object>>,
    width: usize,
    height: usize,

    has_floor: bool,
    floor_level: i32,
}

impl Map {
    fn new(width: usize, height: usize, has_floor: bool) -> Self {
        let mut items = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Object::Empty);
            }
            items.push(row);
        }
        Self {
            items,
            width,
            height,
            has_floor,
            floor_level: 0,
        }
    }

    fn calculate_floor_level(&mut self) {
        let mut max_y = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                match self.items[y][x] {
                    Object::Empty => {}
                    _ => {
                        if y > max_y {
                            max_y = y;
                        }
                    }
                }
            }
        }

        self.floor_level = (max_y + 2) as i32;
    }

    fn get(&self, point: Point) -> Object {
        if self.has_floor && point.y == self.floor_level {
            Object::Sand
        } else {
            self.items[point.y as usize][point.x as usize]
        }
    }

    fn set(&mut self, point: Point, object: Object) {
        self.items[point.y as usize][point.x as usize] = object;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_x = 10000;
        let mut min_y = 10000;
        let mut max_x = 0;
        let mut max_y = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                match self.items[y][x] {
                    Object::Empty => {}
                    _ => {
                        if x < min_x {
                            min_x = x;
                        }
                        if x > max_x {
                            max_x = x;
                        }
                        if y < min_y {
                            min_y = y;
                        }
                        if y > max_y {
                            max_y = y;
                        }
                    }
                }
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let char = match self.items[y][x] {
                    Object::Empty => '.',
                    Object::Rock => '#',
                    Object::Sand => '@',
                };

                f.write_char(char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Object {
    Empty,
    Rock,
    Sand,
}

fn process_input(input: &str, has_floor: bool) -> Map {
    let mut map = Map::new(1000, 1000, has_floor);

    for row in input.split('\n') {
        if row.is_empty() {
            continue;
        }
        let mut start_point: Point = (-1, -1).into();
        for coord_str in row.split("->") {
            let mut coord = coord_str.split(',');
            let x: i32 = coord.next().unwrap().trim().parse().unwrap();
            let y: i32 = coord.next().unwrap().trim().parse().unwrap();
            let new_point: Point = (x, y).into();

            if start_point != (-1, -1).into() {
                let mut point = start_point;
                let delta = (new_point - start_point).normalized_int();

                loop {
                    map.set(point, Object::Rock);
                    if point == new_point {
                        break;
                    }
                    point = point + delta;
                }
            }

            start_point = new_point;
        }
    }

    if has_floor {
        map.calculate_floor_level();
    }

    map
}

#[derive(PartialEq, Eq)]
enum SandResult {
    Still,
    StillAtOrigin,
    FallingOut,
}

fn simulate_sand(map: &mut Map) -> SandResult {
    let mut sand: Point = (500, 0).into();

    let down = (0, 1).into();
    let left = (-1, 1).into();
    let right = (1, 1).into();
    loop {
        if map.get(sand + down) == Object::Empty {
            sand = sand + down;
        } else if map.get(sand + left) == Object::Empty {
            sand = sand + left;
        } else if map.get(sand + right) == Object::Empty {
            sand = sand + right;
        } else {
            map.set(sand, Object::Sand);
            if sand == (500, 0).into() {
                return SandResult::StillAtOrigin;
            } else {
                return SandResult::Still;
            }
        }

        if (sand + down).y >= map.height as i32 {
            return SandResult::FallingOut;
        }
    }
}

fn solve_a(input: &str) -> i32 {
    let mut map = process_input(input, false);

    let mut i = 0;
    while simulate_sand(&mut map) != SandResult::FallingOut {
        i += 1;
    }

    i
}

fn solve_b(input: &str) -> i32 {
    let mut map = process_input(input, true);

    let mut i = 0;
    while simulate_sand(&mut map) != SandResult::StillAtOrigin {
        i += 1;
    }

    i + 1
}
