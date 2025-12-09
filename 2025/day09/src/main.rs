use std::env;
use std::fmt::Display;
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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


// Liang-Barsky
// https://en.wikipedia.org/wiki/Liang%E2%80%93Barsky_algorithm
fn line_intersects_box(p0: Point, p1: Point, xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> bool {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    let mut u1: f64 = 0.0;
    let mut u2: f64 = 1.0;

    for (pi, qi) in [
        (-dx, p0.x - xmin),
        (dx, xmax - p0.x),
        (-dy, p0.y - ymin),
        (dy, ymax - p0.y),
    ] {
        if pi == 0 {
            if qi < 0 {
                return false;
            }
        } else if pi < 0 {
            u1 = ((qi as f64)/(pi as f64)).max(u1);
        } else if pi > 0 {
            u2 = ((qi as f64)/(pi as f64)).min(u2);
        }

    }

    u1 <= u2
}

fn process_input(input: &str) -> Vec<Point> {
    input
        .split("\n")
        .filter_map(|x| {
            let pt: Vec<i64> = x.split(",").flat_map(|y| y.parse()).collect();

            if pt.len() == 2 {
                Some(Point { x: pt[0], y: pt[1] })
            } else {
                None
            }
        })
        .collect()
}

fn solve_a(input: &str) -> i64 {
    let points = process_input(input);

    let mut area_max = 0;
    for px in &points {
        for py in &points {
            let x_dist = (px.x - py.x).abs() + 1;
            let y_dist = (px.y - py.y).abs() + 1;
            let area = x_dist * y_dist;
            if area > area_max {
                area_max = area;
            }
        }
    }

    area_max
}

fn solve_b(input: &str) -> i64 {
    let points = process_input(input);

    let mut area_max = 0;
    for (i, px) in points.iter().enumerate() {
        'outer: for (j, py) in points.iter().enumerate() {
            if j > i {
                continue;
            }
            let x_dist = (px.x - py.x).abs() + 1;
            let y_dist = (px.y - py.y).abs() + 1;
            let area = x_dist * y_dist;

            if area > area_max {
                let xmin = px.x.min(py.x);
                let xmax = px.x.max(py.x);
                let ymin = px.y.min(py.y);
                let ymax = px.y.max(py.y);

                for i in 0..points.len() {
                    let start = if i == 0 {
                        &points[points.len() - 1]
                    } else {
                        &points[i - 1]
                    };
                    let end = &points[i];

                    if line_intersects_box(*start, *end, xmin + 1, xmax - 1, ymin + 1, ymax - 1) {
                        continue 'outer;
                    }
                }

                area_max = area;
            }
        }
    }

    area_max
}

