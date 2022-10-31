use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.trim().split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().chars().collect())
        .collect()
}



fn solve_a(input: &str) -> i32 {
    let points = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    let matching = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<')
    ]);

    let mut score = 0;
    for line in process_input(input) {
        let mut stack = vec![];
        for char in line {
            match char {
                '(' | '[' | '{' | '<' => { stack.push(char); },

                ')' | ']' | '}' | '>' => {
                    if stack.last() != Some(&matching[&char]) {
                        score += points[&char];
                        break;
                    } else {
                        stack.pop();
                    }
                }

                c => panic!("Unexpected character {}", c)
            }
        }
    }

    score
}

fn solve_b(input: &str) -> i64 {
    let points = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);

    let matching = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<')
    ]);

    let mut scores = vec![];
    'outer: for line in process_input(input) {
        let mut stack = vec![];
        for char in line {
            match char {
                '(' | '[' | '{' | '<' => { stack.push(char); },

                ')' | ']' | '}' | '>' => {
                    if stack.last() != Some(&matching[&char]) {
                        continue 'outer;
                    } else {
                        stack.pop();
                    }
                }

                c => panic!("Unexpected character {}", c)
            }
        }

        let mut partial_score: i64 = 0;
        for completing in stack.iter().rev() {
            partial_score *= 5;
            partial_score += points[&completing];
        }

        scores.push(partial_score);
    }

    scores.sort();
    scores[scores.len() / 2]
}
