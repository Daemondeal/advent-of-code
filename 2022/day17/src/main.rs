use std::env;
use std::fmt::{Display, Write};
use std::fs;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use tqdm::Iter;

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

#[derive(Clone)]
struct TetrisMap {
    layers: Vec<[bool; 7]>,
    current_shape: usize,
    bottom_height: usize,
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
            bottom_height: 0,
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
                if *cell == true {
                    self.layers[y + (height - i - 1)][x + j] = *cell;
                }
            }
        }
    }

    fn check_collision(&self, x: i64, y: i64, shape: &[Vec<bool>]) -> bool {
        let height = shape.len() as i64;
        for (i, piece_layer) in shape.iter().enumerate() {
            for (j, cell) in piece_layer.iter().enumerate() {
                if cell == &false {
                    continue;
                }

                let x_pos = x + j as i64;
                let y_pos = y + (height - i as i64 - 1);

                if x_pos >= 7 || x_pos < 0 || y_pos < 0 {
                    return true;
                }

                if self.layers[y_pos as usize][x_pos as usize] == true {
                    return true;
                }
            }
        }

        false
    }

    fn print_with_shape(&self, x: i64, y: i64, shape: &[Vec<bool>]) {
        let mut new_map = (*self).clone();
        new_map.insert_shape(x, y, shape);
        println!("{new_map}");
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

        // 100 should be big enough
        let upper_height_limit = 100;

        loop {
            while self.layers.len() > upper_height_limit {
                self.layers.remove(0);
                self.bottom_height += 1;

                y -= 1;
            }

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
    let mut map = TetrisMap::new();

    let mut move_index = 0;

    for _ in 0..2022 {
        move_index = map.next_shape(&moves, move_index);
    }

    map.get_height() + map.bottom_height
}

fn solve_b(input: &str) -> usize {
    let moves = process_input(input);
    let mut map = TetrisMap::new();

    let mut move_index = 0;

    let mut snapshot = TetrisMap::new();
    let mut snap_len = 0;
    let mut snap_move_index = 0;

    let total_iters = 1000000000000i64;

    // Needs to be a big number, to let the cyclicity settle in
    let starting_snap = 10000;

    let mut remaining_iters = total_iters;
    let mut skipped = 0;

    while remaining_iters > 0 {
        let iter = total_iters - remaining_iters;
        move_index = map.next_shape(&moves, move_index);

        if iter == starting_snap {
            snapshot = map.clone();
            snap_len = snapshot.get_height() + snapshot.bottom_height;
            snap_move_index = move_index;
        } else if iter > starting_snap
            && skipped == 0
            && snapshot.layers == map.layers
            && map.current_shape == snapshot.current_shape
            && move_index == snap_move_index
        {
            // Cycle found

            let delta = map.get_height() + map.bottom_height - snap_len;
            let cycle_length = iter - starting_snap;
            let cycles_skipped =
                ((remaining_iters) as f64 / (cycle_length as f64)).floor() as usize;

            skipped = delta * cycles_skipped;
            remaining_iters -= cycles_skipped as i64 * cycle_length;
        }
        remaining_iters -= 1;
    }

    map.get_height() + map.bottom_height + skipped
}
