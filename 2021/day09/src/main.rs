use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input.split("\n")
        .map(|x| x.chars().map(|x| x.to_digit(10)).flatten().collect::<Vec<u32>>())
        .filter(|x| x.len() > 0)
        .collect()
}

fn solve_a(input: &str) -> i64 {
    let height_map = process_input(input);
    let steps: [(i32, i32); 4] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1)
    ];

    let mut risk: u64 = 0;

    for (i, row) in height_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut is_low = true;

            for (di, dj) in steps {
                if i as i32 + di < 0 || j as i32 + dj < 0 {
                    continue;
                }

                if let Some(adj_row) = height_map.get((i as i32 + di) as usize) {
                    if let Some(adj_cell) = adj_row.get((j as i32 + dj) as usize) {
                        if adj_cell <= cell {
                            is_low = false;
                            break;
                        }
                    }
                }
            }

            if is_low {
                risk += (cell + 1) as u64;
            }
        }
    }

    risk as i64
}

fn solve_b(input: &str) -> i64 {
    let height_map = process_input(input);
    let steps: [(i32, i32); 4] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1)
    ];

    let mut low_points = vec![];    

    for (i, row) in height_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut is_low = true;

            for (di, dj) in steps {
                if i as i32 + di < 0 || j as i32 + dj < 0 {
                    continue;
                }

                if let Some(adj_row) = height_map.get((i as i32 + di) as usize) {
                    if let Some(adj_cell) = adj_row.get((j as i32 + dj) as usize) {
                        if adj_cell <= cell {
                            is_low = false;
                            break;
                        }
                    }
                }
            }
            
            if is_low {
                low_points.push((i as i32, j as i32));
            }
        }
    }

    let mut basins = vec![];

    for (i, j) in low_points {
        let mut visited = HashSet::new();
        basins.push(explore_basin(&height_map, i, j, &mut visited));
    }

    basins.sort();

    basins.iter().rev().take(3).fold(1, |acc, x| acc * (*x as i64))
}


fn explore_basin(map: &[Vec<u32>], i: i32, j: i32, visited: &mut HashSet<(i32, i32)>) -> u32 {
    visited.insert((i, j));

    if i < 0 || j < 0 || i >= map.len() as i32|| j >= map[i as usize].len() as i32 || map[i as usize][j as usize] == 9 {
        return 0;
    }
    
    let steps: [(i32, i32); 4] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1)
    ];    
    
    let mut size = 1;

    for (di, dj) in steps {
        if visited.contains(&(i + di, j + dj)) {
            continue;
        }
        size += explore_basin(map, i + di, j + dj, visited);
    }

    size
}