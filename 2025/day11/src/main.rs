use std::collections::HashMap;
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

fn process_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut res = HashMap::new();
    for line in input.split("\n") {
        let mut raw = line.split(": ");

        let Some(node) = raw.next() else {
            continue;
        };
        let Some(rest) = raw.next() else {
            continue;
        };

        res.insert(
            node.to_owned(),
            rest.split(" ").map(|x| x.to_owned()).collect(),
        );
    }

    res
}

fn count_paths(nodes: &HashMap<String, Vec<String>>, node: &str) -> i64 {
    if node == "out" {
        1
    } else {
        nodes[node].iter().map(|n| count_paths(nodes, n)).sum()
    }
}

fn solve_a(input: &str) -> i64 {
    let nodes = process_input(input);

    if nodes.contains_key("you") {
        count_paths(&nodes, "you")
    } else {
        0
    }
}

fn count_paths_b(
    cache: &mut HashMap<(String, bool, bool), i64>,
    nodes: &HashMap<String, Vec<String>>,
    node: &str,
    dac_found: bool,
    fft_found: bool,
) -> i64 {
    let key = (node.to_owned(), dac_found, fft_found);

    if let Some(v) = cache.get(&key) {
        return *v;
    }

    if node == "out" {
        if dac_found && fft_found { 1 } else { 0 }
    } else {
        let dac_found = dac_found || node == "dac";
        let fft_found = fft_found || node == "fft";

        let res = nodes[node]
            .iter()
            .map(|n| count_paths_b(cache, nodes, n, dac_found, fft_found))
            .sum();

        cache.insert(key, res);

        res
    }
}

fn solve_b(input: &str) -> i64 {
    let nodes = process_input(input);
    let mut cache = HashMap::new();


    if nodes.contains_key("svr") {
        count_paths_b(&mut cache, &nodes, "svr", false, false)
    } else {
        0
    }
}
