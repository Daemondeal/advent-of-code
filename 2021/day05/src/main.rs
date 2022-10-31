use std::collections::HashMap;

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}


fn normalize((x, y): (i32, i32)) -> (i32, i32) {
    let mag = gcd(x.abs(), y.abs());

    (x / mag, y / mag)
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn is_ortho(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn get_points(&self) -> Vec<(i32, i32)> {
        // if self.is_ortho() { return self.get_points_ortho() }
        let dir = normalize((self.x2 - self.x1, self.y2 - self.y1));
        let mut res = vec![];
        let mut x = self.x1;
        let mut y = self.y1;

        while x != self.x2 + dir.0 || y != self.y2 + dir.1 {
            res.push((x, y));

            x += dir.0;
            y += dir.1;
        }

        res
    }
}

fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn parse_point(input: &str) -> Option<(i32, i32)> {
    let mut points = input.split(",").map(|x| x.trim().parse()).flatten();

    if let Some(a) = points.next() {
        if let Some(b) = points.next() {
            return Some((a, b));
        }
    }

    None
}

fn parse_input(input: &str) -> Vec<Line> {
    input.split("\n")
        .map(|line| 
            line.split("->").map(|x| parse_point(x)).flatten().collect::<Vec<(i32, i32)>>()
        )
        .filter(|x| x.len() > 0)
        .map(|points| Line::new(points[0].0, points[0].1, points[1].0, points[1].1))
        .collect::<Vec<Line>>()
}

fn solve_a(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut points = HashMap::new();

    for line in lines {
        if !line.is_ortho() { continue }
        for point in line.get_points() {
            if points.contains_key(&point) {
                points.insert(point, points[&point] + 1);
            } else {
                points.insert(point, 1);
            }
        }
    }

    points.iter().fold(0, |acc, x| if x.1 > &1 { acc + 1 } else { acc })
}

fn solve_b(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut points = HashMap::new();

    for line in lines {
        for point in line.get_points() {
            if points.contains_key(&point) {
                points.insert(point, points[&point] + 1);
            } else {
                points.insert(point, 1);
            }
        }
    }

    points.iter().fold(0, |acc, x| if x.1 > &1 { acc + 1 } else { acc })
}
