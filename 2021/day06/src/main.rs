fn main() {
    let input = include_str!("input.txt");
    
    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn process_input(input: &str) -> [u64;9] {
    let fishes: Vec<usize> = input.split(",").map(|x| x.trim().parse()).flatten().collect();

    let mut res = [0;9];

    for f in fishes {
        res[f] += 1;
    }

    res
}

fn solve_a(input: &str) -> u64 {
    let mut fishes_pool = process_input(input);

    for _ in 0..80 {
        let tmp = fishes_pool[0];
        for i in 1..fishes_pool.len() {
            fishes_pool[i - 1] = fishes_pool[i];
        }

        fishes_pool[8] = tmp;
        fishes_pool[6] += tmp;
    }

    fishes_pool.iter().sum()
}

fn solve_b(input: &str) -> u64 {
    let mut fishes_pool = process_input(input);

    
    for _ in 0..256 {
        let tmp = fishes_pool[0];
        for i in 1..fishes_pool.len() {
            fishes_pool[i - 1] = fishes_pool[i];
        }

        fishes_pool[8] = tmp;
        fishes_pool[6] += tmp;
    }

    fishes_pool.iter().sum()
}
