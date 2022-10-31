use std::collections::HashSet;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }

    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    let input = include_str!("bigtest.txt");
    

    // println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn solve_b(input: &str) -> i32 {
    let mut map = generate_asteroids(input);

    let lookout = map.iter()
        .map(|asteroid| (asteroid, get_seen_asteroids(&map, asteroid).len()))
        .max_by_key(|(_, n)| *n)
        .unwrap().0;

    while map.len() > 1 {
        let seen_set = get_seen_asteroids(&map, lookout);
        let mut seen: Vec<(i32, i32)> = seen_set
            .iter()
            .map(|x| *x)
            .collect();

        seen.sort_by(|a, b| {
            get_angle(*lookout, *a).partial_cmp(&get_angle(*lookout, *b)).unwrap()
        });

        println!("{:?} {}", seen, seen.len());

        break;
    }

    1203
}

fn get_angle(a: (i32, i32), b: (i32, i32)) -> f32 {
    std::f32::consts::PI - ((b.1 - a.1) as f32).atan2((a.0 - b.0) as f32)
}

fn generate_asteroids(input: &str) -> HashSet<(i32, i32)> {
    let mut map = HashSet::new();

    for (i, row) in input.split('\n').enumerate() {
        for (j, cell) in row.chars().enumerate() {
            if cell == '#' {
                map.insert((j as i32, i as i32));
            }
        }
    }

    map
}

fn get_seen_asteroids(map: &HashSet<(i32, i32)>, looker: &(i32, i32)) -> HashSet<(i32, i32)> {
    let mut seen = HashSet::new();

    for target in map.iter() {
        if looker == target {
            continue;
        }
        let segment = (target.0 - looker.0, target.1 - looker.1);
        let magnitude = gcd(segment.0.abs(), segment.1.abs());
        let direction = (segment.0 / magnitude, segment.1 / magnitude);

        let mut pos = *looker;
        pos = (pos.0 + direction.0, pos.1 + direction.1);
        let mut obstructed = false;

        while pos.0 >= 0 && pos.1 >= 0 
            && (pos.0 != target.0 || pos.1 != target.1) 
            && pos.0 < 100 && pos.1 < 100 
        {
            if map.contains(&pos) {
                obstructed = true;
                break;
            }

            pos = (pos.0 + direction.0, pos.1 + direction.1);                
        }

        if !obstructed {
            seen.insert(*target);
        }
    }

    seen
}

fn solve_a(input: &str) -> i32 {
    let map = generate_asteroids(input);

    map.iter()
        .map(|asteroid| get_seen_asteroids(&map, asteroid).len())
        .max()
        .unwrap() as i32
}
