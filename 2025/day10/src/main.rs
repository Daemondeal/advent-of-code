use std::collections::VecDeque;
use std::env;
use std::fs;

use good_lp::Constraint;
use good_lp::Expression;
use good_lp::ProblemVariables;
use good_lp::Solution;
use good_lp::SolutionStatus;
use good_lp::SolverModel;
use good_lp::constraint;
use good_lp::variable;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: cargo run -- [input_file]");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    let a = solve_a(&input);
    let b = solve_b(&input);

    println!("A: {}", a);
    println!("B: {}", b);
}

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_str(s: &str) -> Option<Self> {
        let mut rest = s.trim().to_owned();

        let take = |r: &str, _l, rch| {
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

fn states_match(state1: &[bool], state2: &[bool]) -> bool {
    for (a, b) in state1.iter().zip(state2) {
        if a != b {
            return false;
        }
    }
    true
}

fn solve_machine_a(machine: &Machine) -> i64 {
    let mut queue = VecDeque::new();

    queue.push_back((vec![false; machine.lights.len()], 0));

    while let Some((state, steps)) = queue.pop_front() {
        for button in &machine.buttons {
            let mut state_clone = state.clone();
            for idx in button {
                state_clone[*idx] = !state_clone[*idx];
            }

            if states_match(&state_clone, &machine.lights) {
                return steps + 1;
            }

            queue.push_back((state_clone, steps + 1));
        }
    }

    -1
}

fn solve_a(input: &str) -> i64 {
    process_input(input).iter().map(solve_machine_a).sum()
}

fn solve_machine_b(machine: &Machine) -> i64 {
    let mut problem = ProblemVariables::new();

    let mut vars = vec![];
    let mut obj: Expression = 0.into();
    for _ in 0..machine.buttons.len() {
        let v = problem.add(variable().min(0).integer());

        obj += v;
        vars.push(v);
    }

    let mut expr_const: Vec<Expression> = machine
        .joltage
        .iter()
        .map(|x| *x as i32)
        .map(|x| (-x).into())
        .collect();

    for (i, button) in machine.buttons.iter().enumerate() {
        for j in button {
            expr_const[*j] += vars[i];
        }
    }

    let zero: Expression = 0.into();
    let constraints: Vec<Constraint> = expr_const
        .iter()
        .map(|x| constraint::eq(x.clone(), zero.clone()))
        .collect();

    let solution = problem
        .minimise(obj)
        .using(good_lp::default_solver)
        .with_all(constraints)
        .solve()
        .unwrap();

    assert!(matches!(solution.status(), SolutionStatus::Optimal));

    vars.iter().map(|x| solution.value(*x) as i64).sum()
}

fn solve_b(input: &str) -> i64 {
    process_input(input).iter().map(solve_machine_b).sum()
}
