use std::env;
use std::fmt::{Display, Write};
use std::fs;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;

#[derive(Debug)]
enum MoveDirection {
    Left,
    Right,
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

fn process_input(input: &str) -> Vec<MoveDirection> {
    input
        .trim()
        .as_bytes()
        .iter()
        .map(|x| match x {
            b'>' => MoveDirection::Right,
            b'<' => MoveDirection::Left,
            _ => {
                panic!("Unexpected character {x}");
            }
        })
        .collect()
}

struct TetrisMap {
    layers: Vec<[bool; 7]>,
    current_shape: usize,
}

impl Display for TetrisMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for layer in self.layers.iter().rev() {
            f.write_char('│')?;
            for cell in layer {
                if cell == &true {
                    f.write_char('■')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_str("│\n")?;
        }

        f.write_str("└───────┘")?;

        Ok(())
    }
}

lazy_static! {
    static ref SHAPES: Vec<Vec<Vec<bool>>> = vec![
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];
}

impl TetrisMap {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            current_shape: 0,
        }
    }

    pub fn get_height(&self) -> usize {
        for (i, layer) in self.layers.iter().enumerate() {
            if !layer.contains(&true) {
                return i;
            }
        }

        0
    }

    pub fn insert_shape(&mut self, x: i64, y: i64, shape: &[Vec<bool>]) {
        let height = shape.len();
        for (i, piece_layer) in shape.iter().enumerate() {
            for (j, cell) in piece_layer.iter().enumerate() {
                let (x, y) = (x as usize, y as usize);
                self.layers[y + (height - i - 1)][x + j] = *cell;
            }
        }
    }

    fn check_collision(&self, x: i64, y: i64, shape: &[Vec<bool>]) -> bool {
        let height = shape.len();
        for (i, piece_layer) in shape.iter().enumerate() {
            for (j, cell) in piece_layer.iter().enumerate() {
                if x + j as i64 >= 7 || x + (j as i64) < 0 || y < 0 {
                    return true;
                }
                let (x, y) = (x as usize, y as usize);

                if self.layers[y + (height - i - 1)][x + j] == true && cell == &true {
                    return true;
                }
            }
        }

        false
    }

    pub fn next_shape(&mut self, moves: &[MoveDirection], last_move: usize) -> usize {
        let shape = &SHAPES[self.current_shape];

        let rocks_height = self.get_height();
        let height = shape.len();

        let piece_top = height + rocks_height + 3;

        while self.layers.len() <= piece_top {
            self.layers.push([false; 7]);
        }

        let mut x = 2;
        let mut y = rocks_height as i64 + 3;
        let mut move_index = last_move;

        loop {
            let next_move = &moves[move_index];
            move_index = (move_index + 1) % moves.len();

            let next_x = match next_move {
                MoveDirection::Left => x - 1,
                MoveDirection::Right => x + 1,
            };

            if !self.check_collision(next_x, y, shape) {
                x = next_x;
            }

            let next_y = y - 1;
            if self.check_collision(x, next_y, shape) {
                break;
            }

            y = next_y;
        }

        self.insert_shape(x, y, shape);

        self.current_shape = (self.current_shape + 1) % SHAPES.len();
        move_index
    }
}

fn solve_a(input: &str) -> usize {
    let moves = process_input(input);
    println!("{}", moves.len());
    let mut map = TetrisMap::new();

    let mut move_index = 0;

    for _ in 0..2022 {
        print!("{}[2J", 27 as char);
        move_index = map.next_shape(&moves, move_index);
    }

    map.get_height()
}

fn solve_b(input: &str) -> i32 {
    0
}
