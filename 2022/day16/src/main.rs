use std::collections::BinaryHeap;
use std::env;
use std::fs;

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

fn process_input(input: &str) -> (ValveSystem, usize) {
    let input_re = Regex::new(VALVE_REGEX).unwrap();

    let mut raw_valves = vec![];
    let mut starting = 0;

    for capture in input_re.captures_iter(input) {
        let name = capture[1].to_string();
        let flow_rate: i64 = capture[2].parse().unwrap();
        let connections: Vec<String> = capture[3].split(", ").map(|x| x.to_string()).collect();

        if &name == "AA" {
            starting = raw_valves.len();
        }

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
                .position(|x| x.0 == connection.trim())
                .unwrap();
            system.tunnels[i].push(to);
        }
    }

    (system, starting)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra
fn find_path_length(system: &ValveSystem, from: usize, to: usize) -> Option<i64> {
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
            return Some(cost as i64);
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

fn find_best_path_recursively(
    system: &ValveSystem,
    non_zero_valves: &[usize],
    costs: &[Vec<i64>],
    open_valves: usize,

    human_position: usize,
    elephant_position: usize,

    human_remaining: i64,
    elephant_remaining: i64,

    path_pressure: i64,

    maximum_pressure: &mut i64,
) {
    for human_next in non_zero_valves {
        if human_remaining < elephant_remaining {
            break;
        }
        if open_valves & (1 << human_next) > 0 {
            continue;
        }

        let human_pressure = system.valves[*human_next];
        let human_time_cost = (costs[human_position][*human_next] + 1) as i64;
        let time_human = human_remaining - human_time_cost;

        if human_time_cost > human_remaining {
            continue;
        }
        let next_open_valves = open_valves | 1 << human_next;

        let new_total_pressure = path_pressure + human_pressure * time_human;

        find_best_path_recursively(
            system,
            non_zero_valves,
            costs,
            next_open_valves,
            *human_next,
            elephant_position,
            time_human,
            elephant_remaining,
            new_total_pressure,
            maximum_pressure,
        );
    }

    for elephant_next in non_zero_valves {
        if elephant_remaining < human_remaining {
            break;
        }
        if open_valves & (1 << elephant_next) > 0 {
            continue;
        }

        let elephant_pressure = system.valves[*elephant_next];
        let elephant_time_cost = (costs[elephant_position][*elephant_next] + 1) as i64;
        let time_elephant = elephant_remaining - elephant_time_cost;

        if elephant_time_cost > elephant_remaining {
            continue;
        }
        let next_open_valves = open_valves | 1 << elephant_next;

        let new_total_pressure = path_pressure + elephant_pressure * time_elephant;

        find_best_path_recursively(
            system,
            non_zero_valves,
            costs,
            next_open_valves,
            human_position,
            *elephant_next,
            human_remaining,
            time_elephant,
            new_total_pressure,
            maximum_pressure,
        );
    }

    if path_pressure > *maximum_pressure {
        *maximum_pressure = path_pressure;
    }
}

fn solve_a(input: &str) -> i64 {
    let (system, starting) = process_input(input);

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
            row.push(find_path_length(&system, i, j).unwrap_or(i64::MAX));
        }

        costs.push(row);
    }

    // let mut open_valves = HashSet::new();
    let mut maximum_pressure = 0;

    find_best_path_recursively(
        &system,
        &non_zero_valves,
        &costs,
        0,
        starting,
        0,
        30,
        0,
        0,
        &mut maximum_pressure,
    );

    maximum_pressure
}

fn solve_b(input: &str) -> i64 {
    let (system, starting) = process_input(input);

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
            row.push(find_path_length(&system, i, j).unwrap_or(i64::MAX));
        }

        costs.push(row);
    }

    let mut maximum_pressure = 0;

    find_best_path_recursively(
        &system,
        &non_zero_valves,
        &costs,
        0,
        starting,
        starting,
        26,
        26,
        0,
        &mut maximum_pressure,
    );

    maximum_pressure
}
