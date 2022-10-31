fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn solve_b(input: &str) -> i32 {
    input.split('\n')
        .map(|x| x.parse())
        .flatten()
        .map(fuel_b)
        .sum()
}

fn fuel_b(x: i32) -> i32 {    
    let f = (x / 3) - 2;
    if f < 0 {
        return 0;
    }

    f + fuel_b(f)
}

fn solve_a(input: &str) -> u32 {
    input.split('\n')
    .map(|x| x.parse())
    .flatten()
    .map(|x: u32| (x / 3) - 2)
    .sum()
}
