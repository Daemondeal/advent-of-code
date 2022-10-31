fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input.split('\n')
        .map(|x| x.chars()
            .map(|x| x.to_digit(10))
            .flatten()
            .map(|x| x as i32)
            .collect::<Vec<i32>>()
        )
        .filter(|x| !x.is_empty())
        .collect::<Vec<Vec<i32>>>()
}


fn perform_step(octopi: &mut Vec<Vec<i32>>) {
    // Increase energy levels
    let size = octopi.len();
    for (i, _) in octopi.iter_mut().enumerate() {

    }
}

fn solve_a(input: &str) -> i32 {
    let octopi = process_input(input);

    0
}

fn solve_b(input: &str) -> i32 {
    0
}
