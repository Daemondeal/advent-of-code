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

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

    let mut perimeter = vec![];

    for i in 0..points.len() {
        let start = if i == 0 {
            &points[points.len() - 1]
        } else {
            &points[i - 1]
        };
        let end = &points[i];

        if start.x == end.x {
            let y_start = start.y.min(end.y);
            let y_end = start.y.max(end.y);

            for y in y_start..y_end {
                perimeter.push(Point { x: start.x, y });
            }
        } else {
            let x_start = start.x.min(end.x);
            let x_end = start.x.max(end.x);

            for x in x_start..x_end {
                perimeter.push(Point { x, y: start.y });
            }
        }
    }

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

                for p in perimeter.iter() {
                    if p.x > xmin && p.x < xmax && p.y > ymin && p.y < ymax {
                        continue 'outer;
                    }
                }

                area_max = area;
            }
        }
    }

    // let mut bitmap = [0_u8; 15 * 15];
    // for p in perimeter.iter() {
    //     bitmap[(p.x + p.y * 15) as usize] = 255;
    // }
    // for x in 0..15 {
    //     for y in 0..15 {
    //         let r = bitmap[x + y * 15];
    //         print!("\x1b[38;2;{r};0;0m█");
    //     }
    //     println!();
    // }
    // println!("\x1b[0m");

    area_max
}

fn pixel(r: u8, g: u8, b: u8) {
    print!("\x1b[38;2;{r};{g};{b}m█");
}
