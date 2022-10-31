use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    
    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn binary_to_decimal(binary: &[u32]) -> u32 {
    binary.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| 2u32.pow(i as u32) * x)
        .fold(0, |a, x| a + x)
}

fn read_numbers(input: &str) -> Vec<Vec<u32>> {
    input.split("\n")
        .map(|x| {
            x.chars().map(|x| x.to_digit(10)).flatten().collect::<Vec<u32>>()
        })
        .filter(|x| x.len() > 0)
        .collect::<Vec<Vec<u32>>>()
}

fn solve_a(input: &str) -> u32 {
    let numbers = read_numbers(input);
    
    let len = numbers[0].len();
    let half = numbers.len() / 2;
    
    let bits = numbers.into_iter()
        .fold(vec![0u32;len], |acc, x| {
            acc.iter().zip(x).map(|(a, n)| a + n).collect::<Vec<u32>>()
        })
        .iter()
        .map(|x| if *x > half as u32 { 1 } else { 0 } )
        .collect::<Vec<u32>>();

    let flipped = bits.iter()
        .map(|x| if *x == 0 { 1 } else { 0 })
        .collect::<Vec<u32>>();

    let gamma = binary_to_decimal(&bits);
    let epsilon = binary_to_decimal(&flipped);

    gamma * epsilon
}

#[derive(Clone, Copy, Debug)]
enum MostCommon {
    Zero,
    One,
    Neither
}

fn filter_numbers(numbers: Vec<Vec<u32>>, criteria: impl Fn(MostCommon) -> u32) -> u32 {
    let mut cur = numbers.iter().map(|x| x).collect::<Vec<&Vec<u32>>>();
    let mut i = 0;

    while cur.len() > 1 {
        let count = cur.iter().fold(0, |a, x| a + x[i]) as usize;

        let freq = match count.cmp(&(cur.len() - count)) {
            Ordering::Less    => MostCommon::Zero,
            Ordering::Greater => MostCommon::One,
            Ordering::Equal   => MostCommon::Neither
        };

        cur = cur.iter()
            .filter(|x| x[i] == criteria(freq))
            .map(|x| *x)
            .collect();


        i += 1;
    }

    binary_to_decimal(cur.iter().next().unwrap())
}

fn solve_b(input: &str) -> u32 {
    let numbers = read_numbers(input);

    let oxygen = filter_numbers(numbers.clone(), |x| {
        match x {
            MostCommon::Neither => 1,
            MostCommon::Zero => 0,
            MostCommon::One => 1
        }
    });

    let co2 = filter_numbers(numbers, |x| {
        match x {
            MostCommon::Neither => 0,
            MostCommon::Zero => 1,
            MostCommon::One => 0
        }
    });

    oxygen * co2
}
