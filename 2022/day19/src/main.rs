use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use regex::Regex;

const BLUEPRINT_REGEX: &str = r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.";

#[derive(Debug)]
struct Blueprint {
    id: i64,
    ore_ore_cost: i64,
    clay_ore_cost: i64,
    obsidian_ore_cost: i64,
    obsidian_clay_cost: i64,
    geode_ore_cost: i64,
    geode_obsidian_cost: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Inventory {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
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

fn process_input(input: &str) -> Vec<Blueprint> {
    let input_re = Regex::new(BLUEPRINT_REGEX).unwrap();

    input_re
        .captures_iter(input)
        .map(|c| Blueprint {
            id: c[1].parse().unwrap(),
            ore_ore_cost: c[2].parse().unwrap(),
            clay_ore_cost: c[3].parse().unwrap(),
            obsidian_ore_cost: c[4].parse().unwrap(),
            obsidian_clay_cost: c[5].parse().unwrap(),
            geode_ore_cost: c[6].parse().unwrap(),
            geode_obsidian_cost: c[7].parse().unwrap(),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    inventory: Inventory,
    golems: Inventory,
    // time_left: usize,
}

fn maximize_blueprint(blueprint: &Blueprint, starting_time_left: usize) -> i64 {
    let default_state = State {
        inventory: Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        golems: Inventory {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };

    let mut states = vec![(default_state, starting_time_left)];
    let mut max_geodes = 0;

    let max_ore_usage = blueprint
        .clay_ore_cost
        .max(blueprint.ore_ore_cost)
        .max(blueprint.geode_ore_cost);

    let max_clay_usage = blueprint.obsidian_clay_cost;
    let max_obsidian_usage = blueprint.geode_obsidian_cost;

    // let mut cache = HashMap::new();

    while let Some((state, time_left)) = states.pop() {
        // if let Some(cached_time) = cache.get_mut(&state) {
        //     if *cached_time > time_left {
        //         continue;
        //     } else {
        //         *cached_time = time_left;
        //     }
        // } else {
        //     cache.insert(state.clone(), time_left);
        // }

        let mut made_golem = false;

        let ore = state.inventory.ore + state.golems.ore;
        let clay = state.inventory.clay + state.golems.clay;
        let obsidian = state.inventory.obsidian + state.golems.obsidian;
        let geode = state.inventory.geode + state.golems.geode;

        let next_time_left = time_left - 1;

        if next_time_left == 0 {
            if geode > max_geodes {
                // println!("New maximum found: {geode}");
                max_geodes = geode;
            }
            continue;
        }

        // Geode Robot
        if state.inventory.ore >= blueprint.geode_ore_cost
            && state.inventory.obsidian >= blueprint.geode_obsidian_cost
        {
            states.push((
                State {
                    inventory: Inventory {
                        ore: ore - blueprint.geode_ore_cost,
                        clay,
                        obsidian: obsidian - blueprint.geode_obsidian_cost,
                        geode,
                    },
                    golems: Inventory {
                        ore: state.golems.ore,
                        clay: state.golems.clay,
                        obsidian: state.golems.obsidian,
                        geode: state.golems.geode + 1,
                    },
                },
                next_time_left,
            ));

            // made_golem = true;
            // continue;
        }

        // Ore Robot
        if state.inventory.ore >= blueprint.ore_ore_cost && state.golems.ore < max_ore_usage {
            states.push((
                State {
                    inventory: Inventory {
                        ore: ore - blueprint.ore_ore_cost,
                        clay,
                        obsidian,
                        geode,
                    },
                    golems: Inventory {
                        ore: state.golems.ore + 1,
                        clay: state.golems.clay,
                        obsidian: state.golems.obsidian,
                        geode: state.golems.geode,
                    },
                },
                next_time_left,
            ));
            made_golem = true;
        }

        // Clay Robot
        if state.inventory.ore >= blueprint.clay_ore_cost && state.golems.clay < max_clay_usage {
            states.push((
                State {
                    inventory: Inventory {
                        ore: ore - blueprint.clay_ore_cost,
                        clay,
                        obsidian,
                        geode,
                    },
                    golems: Inventory {
                        ore: state.golems.ore,
                        clay: state.golems.clay + 1,
                        obsidian: state.golems.obsidian,
                        geode: state.golems.geode,
                    },
                },
                next_time_left,
            ));

            made_golem = true;
        }

        // Obisidan Robot
        if state.inventory.ore >= blueprint.obsidian_ore_cost
            && state.inventory.clay >= blueprint.obsidian_clay_cost
            && state.golems.obsidian < max_obsidian_usage
        {
            states.push((
                State {
                    inventory: Inventory {
                        ore: ore - blueprint.obsidian_ore_cost,
                        clay: clay - blueprint.obsidian_clay_cost,
                        obsidian,
                        geode,
                    },
                    golems: Inventory {
                        ore: state.golems.ore,
                        clay: state.golems.clay,
                        obsidian: state.golems.obsidian + 1,
                        geode: state.golems.geode,
                    },
                },
                next_time_left,
            ));

            made_golem = true;
        }

        // Do nothing
        // This made_golem optimization works on the input for some reason
        // oh well
        // if !made_golem {
        states.push((
            State {
                inventory: Inventory {
                    ore,
                    clay,
                    obsidian,
                    geode,
                },
                golems: state.golems,
            },
            next_time_left,
        ));
        // }
    }

    max_geodes
}

fn solve_a(input: &str) -> i64 {
    let blueprints = process_input(input);

    let quality = blueprints
        .par_iter()
        .map(|bp| bp.id * maximize_blueprint(bp, 24))
        .sum();

    quality
}

fn solve_b(input: &str) -> i64 {
    let blueprints = process_input(input).into_iter().take(3).collect::<Vec<_>>();

    let quality = blueprints
        .par_iter()
        .map(|bp| maximize_blueprint(bp, 32))
        .collect::<Vec<_>>();

    quality.iter().fold(1, |acc, val| acc * val)
}
