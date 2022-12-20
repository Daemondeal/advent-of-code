use std::collections::VecDeque;
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

fn process_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split('\n')
        .map(|x| x.parse())
        .flatten()
        .collect()
}

fn solve(original_list: &[i64], key: i64, rounds: usize) -> i64 {
    let size = original_list.len();
    let mut modified_list = original_list
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x * key))
        .collect::<VecDeque<_>>();

    for _ in 0..rounds {
        for original_index in 0..original_list.len() {
            let index = modified_list
                .iter()
                .position(|(i, _)| i == &original_index)
                .unwrap();

            let item = modified_list[index].1;

            modified_list.rotate_left(index);
            modified_list.pop_front();

            modified_list.rotate_left(item.rem_euclid((size - 1) as i64) as usize);
            modified_list.push_front((original_index, item));
        }
    }

    let zero_pos = modified_list.iter().position(|(_, x)| *x == 0).unwrap();

    modified_list[(zero_pos + 1000) % size].1
        + modified_list[(zero_pos + 2000) % size].1
        + modified_list[(zero_pos + 3000) % size].1
}

fn solve_a(input: &str) -> i64 {
    let original_list = process_input(input);

    solve(&original_list, 1, 1)
}

fn solve_b(input: &str) -> i64 {
    let original_list = process_input(input);

    solve(&original_list, 811589153, 10)
}
