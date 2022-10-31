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

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|x| x.split("").map(|y| y.parse()).flatten().collect())
        .collect::<Vec<Vec<i32>>>()
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect()
}

fn get_neighbours(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut neighs = vec![];

    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            let nx = (x as i32) + dx;
            let ny = (y as i32) + dy;

            if !(dx == 0 && dy == 0) && nx >= 0 && ny >= 0 && nx < w as i32 && ny < h as i32 {
                neighs.push((nx as usize, ny as usize))
            }
        }
    }

    neighs
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for line in grid {
        println!(
            "{}",
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

fn do_step(grid: &mut Vec<Vec<i32>>) -> i32 {
    let w = grid.len();
    assert!(w > 0);
    let h = grid[0].len();
    assert!(h > 0);

    // Increase
    for x in 0..w {
        for y in 0..h {
            grid[x][y] += 1;
        }
    }

    let mut flashed = true;
    let mut flashes = 0;

    // Flash
    while flashed {
        flashed = false;

        for x in 0..w {
            for y in 0..h {
                if grid[x][y] > 9 {
                    flashed = true;
                    flashes += 1;

                    grid[x][y] = -1;

                    for (nx, ny) in get_neighbours(x, y, w, h) {
                        if grid[nx][ny] >= 0 {
                            grid[nx][ny] += 1;
                        }
                    }
                }
            }
        }
    }

    // Reset
    for x in 0..w {
        for y in 0..h {
            if grid[x][y] < 0 {
                grid[x][y] = 0;
            }
        }
    }

    flashes
}

fn solve_a(input: &str) -> i32 {
    let mut grid = process_input(input);
    let mut flashes = 0;

    print_grid(&grid);

    for _ in 0..100 {
        flashes += do_step(&mut grid);
    }

    flashes
}

fn solve_b(input: &str) -> i32 {
    let mut grid = process_input(input);

    print_grid(&grid);
    let mut i = 1;
    loop {
        let flashes = do_step(&mut grid);
        if flashes == 10 * 10 {
            return i;
        }

        i += 1;
    }
}
