use std::collections::HashSet;
use std::env;
use std::fs;

use tqdm::Iter;

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

fn process_input(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut tokens = line.split(",");

            (
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn solve_a(input: &str) -> usize {
    let cubes = process_input(input);
    let mut free_faces = vec![];

    for (x, y, z) in &cubes {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let new_pos = (x + dx, y + dy, z + dz);

            if !cubes.contains(&new_pos) {
                free_faces.push(new_pos);
            }
        }
    }

    free_faces.len()
}

fn can_escape(
    visited: &mut HashSet<(i32, i32, i32)>,
    cubes: &HashSet<(i32, i32, i32)>,
    mx: i32,
    my: i32,
    mz: i32,
    x: i32,
    y: i32,
    z: i32,
) -> bool {
    let mut stack = vec![(x, y, z)];

    while let Some((x, y, z)) = stack.pop() {
        if cubes.contains(&(x, y, z)) || visited.contains(&(x, y, z)) {
            continue;
        }
        visited.insert((x, y, z));

        if x > mx || y > my || z > mz || x < 0 || y < 0 || z < 0 {
            return true;
        }

        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            stack.push((x + dx, y + dy, z + dz));
        }
    }

    false
}

fn solve_b(input: &str) -> usize {
    let cubes = process_input(input);
    let mut free_faces = vec![];

    let mx = cubes.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let my = cubes.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let mz = cubes.iter().max_by(|x, y| x.2.cmp(&y.2)).unwrap().2;

    let mut visited = HashSet::new();
    for (x, y, z) in cubes.iter() {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let new_pos = (x + dx, y + dy, z + dz);

            if can_escape(
                &mut visited,
                &cubes,
                mx,
                my,
                mz,
                new_pos.0,
                new_pos.1,
                new_pos.2,
            ) {
                free_faces.push(new_pos);
            }
            visited.clear();
        }
    }

    free_faces.len()
}
