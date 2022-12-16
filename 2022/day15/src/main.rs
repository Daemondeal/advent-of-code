pub mod point;

use std::collections::HashSet;
use std::env;
use std::fs;

use point::Point;
use regex::Regex;

const SENSOR_REGEX: &str =
    r"Sensor at x=([0-9\-]+), y=([0-9\-]+): closest beacon is at x=([0-9\-]+), y=([0-9\-]+)";

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
    distance: i64,
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

fn process_input(input: &str) -> Vec<Sensor> {
    let input_re = Regex::new(SENSOR_REGEX).unwrap();
    let mut result = vec![];

    for captures in input_re.captures_iter(input) {
        let xs = captures[1].parse().unwrap();
        let ys = captures[2].parse().unwrap();
        let xb = captures[3].parse().unwrap();
        let yb = captures[4].parse().unwrap();

        let sensor = (xs, ys).into();
        let beacon = (xb, yb).into();

        result.push(Sensor {
            position: sensor,
            beacon,
            distance: sensor.taxicab_distance(&beacon),
        })
    }

    result
}

fn solve_a(input: &str) -> i32 {
    let sensors = process_input(input);

    let y_test: i64 = 2000000;
    let mut min_x = None;
    let mut max_x = None;

    let mut beacon_positions = HashSet::new();

    for sensor in &sensors {
        if let Some(x) = min_x {
            if x > sensor.position.x - sensor.distance {
                min_x = Some(sensor.position.x - sensor.distance);
            }
        } else {
            min_x = Some(sensor.position.x - sensor.distance);
        }
        if let Some(x) = max_x {
            if x < sensor.position.x + sensor.distance {
                max_x = Some(sensor.position.x + sensor.distance);
            }
        } else {
            max_x = Some(sensor.position.x + sensor.distance);
        }

        if sensor.beacon.y == y_test {
            beacon_positions.insert(sensor.beacon.x);
        }
    }

    let min_x = min_x.unwrap();
    let max_x = max_x.unwrap();

    let mut count = 0;

    for x in min_x..=max_x {
        if beacon_positions.contains(&x) {
            continue;
        }
        let point = (x, y_test).into();
        for sensor in &sensors {
            if sensor.position.taxicab_distance(&point) <= sensor.distance {
                count += 1;
                break;
            }
        }
    }

    count
}

fn range_contains_point((lower, upper): &(i64, i64), point: i64) -> bool {
    point >= *lower && point <= *upper
}

fn range_fully_contained((l1, u1): &(i64, i64), (l2, u2): &(i64, i64)) -> bool {
    l1 <= l2 && u1 >= u2
}

fn solve_b(input: &str) -> i64 {
    let sensors = process_input(input);

    let mut beacon_positions = HashSet::new();

    for sensor in &sensors {
        beacon_positions.insert(sensor.beacon);
    }

    let min_x = 0;
    let min_y = 0;
    let max_x = 4000000;
    let max_y = 4000000;

    for y in min_y..=max_y {
        let mut ranges = vec![];
        for sensor in &sensors {
            let width = sensor.distance - (sensor.position.y - y).abs();
            if width > 0 {
                ranges.push((sensor.position.x - width, sensor.position.x + width));
            }
        }

        ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));

        for i in (0..ranges.len()).rev() {
            for j in 0..ranges.len() {
                if ranges[i] != ranges[j] && range_fully_contained(&ranges[j], &ranges[i]) {
                    ranges.remove(i);
                    break;
                }
            }
        }

        for window in ranges.windows(2) {
            let r1 = window[0];
            let r2 = window[1];

            let outside_x = r1.1 + 1;

            if !range_contains_point(&r2, outside_x) && outside_x >= min_x && outside_x <= max_x {
                return outside_x * 4000000 + y;
            }
        }
    }

    -1
}
