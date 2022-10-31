#![allow(dead_code)]
#![allow(clippy::vec_box)]

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

fn main() {

    let input = include_str!("input.txt");

    println!("{}", solve_a(input));
    println!("{}", solve_b(input));
}

fn solve_b(input: &str) -> i32 {
    let root = construct_tree(input);

    calc_min_dist(&root, "YOU", "SAN", 0).unwrap()
}

fn calc_min_dist(current: &OrbitNode, a: &str, b: &str, depth: i32) -> Option<i32> {
    let search_a = search_for(current, a, depth);
    let search_b = search_for(current, b, depth);

    for node in &current.children {
        if let Some(answer) = calc_min_dist(node, a, b, depth + 1) {
            return Some(answer);
        }
    }

    if let Some(depth_a) = search_a {
        if let Some(depth_b) = search_b {
            return Some(depth_a + depth_b - 2 * depth);
        }
    }

    None
}

fn search_for(current: &OrbitNode, name: &str, depth: i32) -> Option<i32> {
    if current.has_child(name) {
        return Some(depth);
    }

    for node in &current.children {
        if let Some(res) = search_for(node, name, depth + 1) {
            return Some(res);
        }
    }

    None
}

fn search_tree<'a>(current: &'a mut Box<OrbitNode>, name: &str) -> Option<&'a mut Box<OrbitNode>> {
    if current.name == name {
        return Some(current);
    }

    for node in current.children.iter_mut() {
        if let Some(res) = search_tree(node, name) {
            return Some(res);
        }
    }

    None
}

fn solve_a(input: &str) -> i32 {
    let root = construct_tree(input);
    root.count_orbit(0)
}

struct OrbitNode {
    name: String,
    children: Vec<Box<OrbitNode>>,
}

impl OrbitNode {
    fn new(name: &str) -> Self {
        OrbitNode { 
            name: name.to_string(),
            children: vec![] 
        }
    }

    fn has_child(&self, name: &str) -> bool {
        for child in &self.children {
            if child.name == name {
                return true;
            }
        }

        false
    }

    fn add_to_orbit(&mut self, orbited: Box<OrbitNode>){
        self.children.push(orbited);
    }

    fn show(&self, indentation: i32) {
        for _ in 0..indentation {
            print!("-");
        }

        println!("{}", self.name);
        for child in &self.children {
            child.show(indentation + 1);
        }
    }

    fn count_orbit(&self, path_len: i32) -> i32 {
        let sum: i32 = self.children.iter().map(|x| x.count_orbit(path_len + 1)).sum();
        path_len + sum
    }
}

fn populate_tree(current: &mut Box<OrbitNode>, map: &HashMap<&str, Vec<&str>>) {
    if let Some(res) = map.get(&*current.name) {
        for orbiting_name in res {
            let mut new_planet = Box::new(OrbitNode::new(orbiting_name));
            populate_tree(&mut new_planet, map);

            current.children.push(new_planet);
        }
    }
}

fn construct_tree(input: &str) -> Box<OrbitNode> {
    let mut root = Box::new(OrbitNode::new("COM"));

    let filtered_input = input
        .split('\n')
        .map(|x| x.trim_end().trim_start())
        .filter(|x| !x.is_empty());

    let mut planet_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for planet in filtered_input {
        let mut tokens = planet.split(')');

        let orbiting_name = tokens.next().unwrap();
        let planet_name = tokens.next().unwrap();

        if let Some(arr) = planet_map.get_mut(orbiting_name) {
            arr.push(planet_name);
        } else {
            planet_map.insert(orbiting_name, vec![planet_name]);
        }
    }

    populate_tree(&mut root, &planet_map);

    root
}
