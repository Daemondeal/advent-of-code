fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn solve_a(input: &str) -> i32 {
    let mut depth = 0;
    let mut h_position = 0;

    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();

        if params.len() == 0 { continue }

        let name = params[0];
        let value: i32 = params[1].parse().unwrap();

        match name {
            "forward" => { h_position += value }
            "down" => { depth += value }
            "up" => { depth -= value }
            _ => {}
        };
        
    }

    depth * h_position
}

fn solve_b(input: &str) -> i32 {
    let mut depth = 0;
    let mut h_position = 0;
    let mut aim = 0;

    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();

        if params.len() == 0 { continue }

        let name = params[0];
        let value: i32 = params[1].parse().unwrap();

        match name {
            "forward" => { 
                h_position += value;
                depth += aim * value;
            }
            "down" => { aim += value }
            "up" => { aim -= value }
            _ => {}
        };
        
    }

    depth * h_position
}
