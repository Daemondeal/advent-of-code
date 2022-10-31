fn main() {
    let input = include_str!("input.txt");

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn load_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn solve_b(input: &str) -> i32 {
    load_input(input)
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|x| x[1] > x[0])
        .fold(0, |a, x| if x { a + 1 } else { a } )
}

fn solve_a(input: &str) -> i32 {
    load_input(input)
        .windows(2)
        .map(|x| x[1] > x[0])
        .fold(0, |a, x| if x { a + 1 } else { a })
}

