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

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist_sqr(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn process_input(input: &str) -> Vec<Point> {
    input
        .split("\n")
        .map(|x| {
            let pt: Vec<i64> = x.split(",").map(|y| y.parse()).flatten().collect();

            if pt.len() == 3 {
                Some(Point {
                    x: pt[0],
                    y: pt[1],
                    z: pt[2],
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn solve_a(input: &str) -> i64 {
    let points = process_input(input);

    let mut connections = HashSet::new();

    let mut circuits: Vec<HashSet<usize>> = (0..points.len())
        .map(|x| {
            let mut s = HashSet::new();
            s.insert(x);
            s
        })
        .collect();

    let max_conn = if circuits.len() > 20 {
        1000
    } else {
        10
    };

    let mut distances = vec![];

    for (i, px) in points.iter().enumerate() {
        for (j, py) in points.iter().enumerate() {
            // Prevent duplicates
            if i > j {
                distances.push(((i, j), px.dist_sqr(py)));
            }
        }
    }

    distances.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

    let mut dist_iter = distances.iter();
    let mut connections_made = 1;
    while connections_made < max_conn {
        let (smallest, _) = dist_iter.next().unwrap();

        connections.insert(*smallest);

        let (i, j) = *smallest;
        let (i_set_idx, _) = circuits
            .iter()
            .enumerate()
            .find(|(_i, x)| x.contains(&i))
            .expect("no set contains i");

        let (mut j_set_idx, _) = circuits
            .iter()
            .enumerate()
            .find(|(_i, x)| x.contains(&j))
            .expect("no set contains j");

        connections_made += 1;
        if i_set_idx == j_set_idx {
            continue;
        }

        let mut i_set = circuits.swap_remove(i_set_idx);
        if j_set_idx >= circuits.len() {
            j_set_idx = i_set_idx;
        }

        let j_set = circuits.swap_remove(j_set_idx);

        i_set.extend(&j_set);
        circuits.push(i_set);

    }

    let mut x: Vec<usize> = circuits.iter().map(|x| x.len()).collect();

    x.sort_by(|x, y| y.cmp(x));

    (x[0] * x[1] * x[2]) as i64
}

fn solve_b(input: &str) -> i64 {
    let points = process_input(input);

    let mut connections = HashSet::new();

    let mut circuits: Vec<HashSet<usize>> = (0..points.len())
        .map(|x| {
            let mut s = HashSet::new();
            s.insert(x);
            s
        })
        .collect();

    let mut distances = vec![];

    for (i, px) in points.iter().enumerate() {
        for (j, py) in points.iter().enumerate() {
            // Prevent duplicates
            if i > j {
                distances.push(((i, j), px.dist_sqr(py)));
            }
        }
    }

    distances.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

    let mut dist_iter = distances.iter();

    let mut last_connection = (0, 0);

    while circuits.len() > 1 {
        let (smallest, _) = dist_iter.next().unwrap();

        last_connection = *smallest;

        connections.insert(*smallest);

        let (i, j) = *smallest;
        let (i_set_idx, _) = circuits
            .iter()
            .enumerate()
            .find(|(_i, x)| x.contains(&i))
            .expect("no set contains i");

        let (mut j_set_idx, _) = circuits
            .iter()
            .enumerate()
            .find(|(_i, x)| x.contains(&j))
            .expect("no set contains j");

        if i_set_idx == j_set_idx {
            continue;
        }

        let mut i_set = circuits.swap_remove(i_set_idx);
        if j_set_idx >= circuits.len() {
            j_set_idx = i_set_idx;
        }

        let j_set = circuits.swap_remove(j_set_idx);

        i_set.extend(&j_set);
        circuits.push(i_set);

    }

    let lc1 = &points[last_connection.0];
    let lc2 = &points[last_connection.1];

    lc1.x * lc2.x
}
