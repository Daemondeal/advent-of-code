use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn process_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut res = vec![];

    for line in input.split("\n") {
        let mut message = line.trim().split("|");

        if let Some(left) = message.next() {
            if let Some(right) = message.next() {
                res.push((
                    left.split(" ").filter(|x| x.len() > 0).collect(), 
                    right.split(" ").filter(|x| x.len() > 1).collect()
                ));
            }
        }
    }

    res
}

fn solve_a(input: &str) -> i32 {
    let numbers = process_input(input);

    let unique = vec![
        2, 
        4,
        3,
        7
    ];
    let mut count = 0;

    for (_, msg) in numbers {
        for n in msg {
            if unique.contains(&n.len()) {
                count += 1;
            }
        }
    }
    
    count
}

fn solve_b(input: &str) -> i32 {
    let numbers: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> = 
        process_input(input).iter()
        .map(|(p, m)| (
            p.iter().map(|x| x.chars().collect::<HashSet<char>>()).collect(), 
            m.iter().map(|x| x.chars().collect::<HashSet<char>>()).collect(), 
        ))
        .collect();

    let mut total = 0;

    for (mut pattern, message) in numbers {
        let mut mappings = vec![HashSet::new(); 10];

        pattern.retain(|p| match p.len() {
            2 => {
                mappings[1] = p.clone();
                false
            },

            4 => {
                mappings[4] = p.clone();
                false
            },

            3 => {
                mappings[7] = p.clone();
                false
            }

            7 => {
                mappings[8] = p.clone();
                false
            },

            _ => true
        });

        pattern.retain(|p| {
            if p.difference(&mappings[1]).count() == 5 {
                mappings[6] = p.clone();
                false
            } else {
                true
            }
        });

        pattern.retain(|p| {
            if p.difference(&mappings[6]).count() == 0 {
                mappings[5] = p.clone();
                false
            } else {
                true
            }
        });

        pattern.retain(|p| {
            if p.difference(&mappings[1]).count() == 3 {
                mappings[3] = p.clone();
                false
            } else {
                true
            }
        });

        pattern.retain(|p| {
            if p.difference(&mappings[4]).count() == 2 {
                mappings[9] = p.clone();
                false
            } else {
                true
            }
        });

        pattern.retain(|p| {
            if p.difference(&mappings[3]).count() == 1 {
                mappings[2] = p.clone();
                false
            } else {
                true
            }
        });

        mappings[0] = pattern.iter().next().unwrap().clone();

        let match_mapping = |mapping: &HashSet<char>| {
            for (i, m) in (&mappings).iter().enumerate() {
                if m == mapping {
                    return i;
                }
            }
            return 10;
        };

        let mut decoded = 0;
        for (i, digit) in message.iter().rev().enumerate() {
            decoded += match_mapping(digit) * 10usize.pow(i as u32);
        }

        total += decoded;
    }

    total as i32
}
