fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.trim().parse()).flatten().collect()
}

fn crab_distance(a: i32, b: i32) -> i32 {
    let x = (a-b).abs();

    x * (x + 1) / 2
}

fn total_distance(crabs: &[i32], dest: i32, dist_function: impl Fn(i32, i32) -> i32) -> i32 {
    crabs.iter().fold(0, |acc, x| acc + dist_function(*x, dest))
}

fn solve_a(input: &str) -> i32 {
    let crabs = parse_input(input);

    let (min, max) = (crabs.iter().min().unwrap(), crabs.iter().max().unwrap());

    let mut dist = total_distance(&crabs, *max, |a, b| (a-b).abs());

    for pos in *min..*max {
        let cur_dist = total_distance(&crabs, pos, |a, b| (a-b).abs());
        if cur_dist < dist {
            dist = cur_dist;
        }
    }

    dist
}

fn solve_b(input: &str) -> i32 {
    let crabs = parse_input(input);

    let (min, max) = (crabs.iter().min().unwrap(), crabs.iter().max().unwrap());

    let mut dist = total_distance(&crabs, *max, crab_distance);

    for pos in *min..*max {
        let cur_dist = total_distance(&crabs, pos, crab_distance);
        if cur_dist < dist {
            dist = cur_dist;
        }
    }

    dist
}
