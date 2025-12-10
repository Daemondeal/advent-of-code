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

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_str(s: &str) -> Option<Self> {
        let mut rest = s.trim().to_owned();

        let take = |r: &str, l, rch| {
            let e = r.find(rch)?;
            Some((r[1..e].to_owned(), r[e + 1..].trim().to_owned()))
        };

        // lights
        let (l, r) = take(&rest, '[', ']')?;
        let lights = l
            .chars()
            .map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect::<Option<_>>()?;
        rest = r;

        // buttons
        let mut buttons = Vec::new();
        while rest.starts_with('(') {
            let (g, r) = take(&rest, '(', ')')?;
            buttons.push(if g.is_empty() {
                vec![]
            } else {
                g.split(',')
                    .map(|x| x.trim().parse().ok())
                    .collect::<Option<_>>()?
            });
            rest = r;
        }

        // joltage
        let (j, _) = take(&rest, '{', '}')?;
        let joltage = j
            .split(',')
            .map(|x| x.trim().parse().ok())
            .collect::<Option<_>>()?;

        Some(Self {
            lights,
            buttons,
            joltage,
        })
    }
}

fn process_input(input: &str) -> Vec<Machine> {
    input.split("\n").flat_map(Machine::from_str).collect()
}

fn solve_machine_a(machine: &Machine) -> i64 {
    let mut queue = VecDeque::new();

    for button in &machine.buttons {
        let mut state = vec![false; machine.lights.len()];

        for idx in button {
            state[*idx] = !state[*idx];
        }

        queue.push_back((state, 1));
    }

    while let Some((state, steps)) = queue.pop_front() {
        for button in &machine.buttons {
            let mut state_clone = state.clone();
            for idx in button {
                state_clone[*idx] = !state_clone[*idx];
            }

            if state_clone.iter().zip(machine.lights.iter()).filter(|(a, b)| a != b).next().is_none() {
                return steps+1;
            }

            queue.push_back((state_clone, steps+1));
        }
    }

    -1
}

fn solve_a(input: &str) -> i64 {
    let machine = process_input(input);

    machine.iter().map(solve_machine_a).sum()
}

fn solve_b(input: &str) -> i64 {
    0
}
