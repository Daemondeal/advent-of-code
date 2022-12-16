use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

use regex::Regex;

const VALVE_REGEX: &str =
    r"Valve ([A-Z]+) has flow rate=(\d+); tunnel(?:s)? lead(?:s)? to valve(?:s)? ([A-Z, ]+)";

#[derive(Debug)]
struct ValveSystem {
    valves: Vec<i64>,
    tunnels: Vec<Vec<usize>>,
}

impl ValveSystem {
    fn new() -> Self {
        Self {
            valves: vec![],
            tunnels: vec![],
        }
    }
}

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

fn process_input(input: &str) -> ValveSystem {
    let input_re = Regex::new(VALVE_REGEX).unwrap();

    let mut raw_valves = vec![];

    for capture in input_re.captures_iter(input) {
        let name = capture[1].to_string();
        let flow_rate: i64 = capture[2].parse().unwrap();
        let connections: Vec<String> = capture[3].split(", ").map(|x| x.to_string()).collect();
        raw_valves.push((name, flow_rate, connections));
    }

    let mut system = ValveSystem::new();

    for (_, flow_rate, _) in &raw_valves {
        system.valves.push(*flow_rate);
        system.tunnels.push(vec![]);
    }

    for (i, (_, _, connections)) in raw_valves.iter().enumerate() {
        for connection in connections {
            let to = raw_valves
                .iter()
                .position(|x| &x.0 == connection.trim())
                .unwrap();
            system.tunnels[i].push(to);
        }
    }

    system
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra
fn find_path(system: &ValveSystem, from: usize, to: usize) -> Option<usize> {
    if from == to {
        return Some(0);
    }

    let vertices = system.valves.len();

    let mut dist: Vec<_> = (0..vertices).map(|_| usize::MAX).collect();
    let mut queue = BinaryHeap::new();

    dist[from] = 0;
    queue.push(State {
        cost: 0,
        position: from,
    });

    while let Some(State { cost, position }) = queue.pop() {
        if position == to {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &system.tunnels[position] {
            let next = State {
                cost: cost + 1,
                position: *edge,
            };

            if next.cost < dist[next.position] {
                queue.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn paths_rec(
    system: &ValveSystem,
    non_zero_valves: &[usize],
    costs: &[Vec<usize>],
    open_valves: &mut HashSet<usize>,
    time_remaining: usize,
    position: usize,
    accumulated_pressure: usize,
) -> usize {
    let mut max_val = 0;

    for next in non_zero_valves {
        if open_valves.contains(next) {
            continue;
        }
        let time_cost = costs[position][*next] + 1;

        if time_remaining >= time_cost {
            open_valves.insert(*next);
            let pressure = system.valves[*next] as usize;
            let remaining_after_valve = time_remaining - time_cost;

            max_val = (paths_rec(
                system,
                non_zero_valves,
                costs,
                open_valves,
                remaining_after_valve,
                *next,
                accumulated_pressure,
            ) + pressure * remaining_after_valve)
                .max(max_val);

            open_valves.remove(next);
        }
    }

    max_val
}

fn solve_a(input: &str) -> usize {
    let system = process_input(input);

    let non_zero_valves: Vec<_> = system
        .valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == 0 { None } else { Some(i) })
        .collect();

    let mut costs = vec![];
    for i in 0..system.valves.len() {
        let mut row = vec![];
        for j in 0..system.valves.len() {
            row.push(find_path(&system, i, j).unwrap_or(usize::MAX));
        }

        costs.push(row);
    }

    let mut open_valves = HashSet::new();

    let start = Instant::now();
    let best = paths_rec(
        &system,
        &non_zero_valves,
        &costs,
        &mut open_valves,
        30,
        0,
        0,
    );

    println!("Time Taken: {:?}", Instant::now() - start);

    best
}

fn solve_b(_input: &str) -> i32 {
    0
}
