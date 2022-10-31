use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("input.txt");

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn norm1(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()    
}

fn solve_b(input: &str) -> i32 {
    let mut moves = input.split('\n');
    
    let first_path = get_path(moves.next().unwrap());
    let second_path = get_path(moves.next().unwrap());

    first_path.0.intersection(&second_path.0)
        .map(|x| first_path.1[x] + second_path.1[x])        
        .min()
        .unwrap()
}

fn solve_a(input: &str) -> i32 {
    let mut moves = input.split('\n');
    
    let first_path = get_path(moves.next().unwrap());
    let second_path = get_path(moves.next().unwrap());

    first_path.0.intersection(&second_path.0)
        .map(|x| norm1(x.0, x.1))
        .min()
        .unwrap()
}

fn get_path(moves: &str) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut path = HashSet::new();
    let mut path_dist = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut total_dist = 0;

    for step in moves.split(',') {
        let mut chars_iter = step.chars();

        let dir = match chars_iter.next() {
            Some('R') => (1, 0),
            Some('L') => (-1, 0),
            Some('U') => (0, 1),
            Some('D') => (0, -1),
            None => (0, 0),
            
            Some(x) => {
                panic!("Invalid direction {}!", x);
            }
        };
        
        let amount = chars_iter
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap();

        for _ in 0..amount {
            x += dir.0;
            y += dir.1;
            path.insert((x, y));
            total_dist += 1;
            path_dist.insert((x, y), total_dist);
        }        
    }

    (path, path_dist)
}