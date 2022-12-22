mod point;

use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Write};
use std::fs;

use point::Point;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Wall,
    Empty,
    NonExisting,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Move {
    Forward(i64),
    TurnClockwise,
    TurnCounterclockwise,
}

struct WrappingMap {
    tiles: Vec<Vec<Tile>>,
}

const WRAPPINGS: [[(usize, usize); 4]; 6] = [
    [(1, 0), (2, 1), (4, 0), (5, 0)], // 1
    [(3, 2), (2, 2), (0, 2), (5, 3)], // 2
    [(1, 3), (3, 1), (4, 1), (0, 3)], // 3
    [(1, 2), (5, 2), (4, 2), (2, 3)], // 4
    [(3, 0), (5, 1), (0, 0), (2, 0)], // 5
    [(3, 3), (1, 1), (0, 1), (4, 3)], // 6
];

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

const FACE_TOP_LEFT: [Point; 6] = [
    Point { x: 50, y: 0 },
    Point { x: 100, y: 0 },
    Point { x: 50, y: 50 },
    Point { x: 50, y: 100 },
    Point { x: 0, y: 100 },
    Point { x: 0, y: 150 },
];

impl WrappingMap {
    pub fn from_str(input_map: &str) -> Self {
        let lines: Vec<_> = input_map.split('\n').collect();

        let (width, height) = (lines.iter().map(|l| l.len()).max().unwrap(), lines.len());

        let mut tiles = vec![vec![Tile::NonExisting; width]; height];

        for (i, row) in lines.iter().enumerate() {
            for (j, cell) in row.as_bytes().iter().enumerate() {
                match cell {
                    b'#' => tiles[i][j] = Tile::Wall,
                    b'.' => tiles[i][j] = Tile::Empty,
                    _ => {}
                }
            }
        }

        Self { tiles }
    }

    pub fn get_starting_position(&self) -> Point {
        for (x, cell) in self.tiles[0].iter().enumerate() {
            if cell == &Tile::Empty {
                return (x, 0).into();
            }
        }

        panic!("Not a valid map")
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_face(&self, point: Point) -> Option<usize> {
        for (i, top_left) in FACE_TOP_LEFT.iter().enumerate() {
            if (top_left.x..(top_left.x + 50)).contains(&point.x)
                && (top_left.y..(top_left.y + 50)).contains(&point.y)
            {
                return Some(i);
            }
        }

        None
    }

    pub fn get_next_point_cube(&self, point: Point, direction: usize) -> (Point, Tile, usize) {
        let mut next_point = point + DIRECTIONS[direction];
        let mut next_direction = direction;

        let (nx, ny) = next_point.into();

        let prev_face = self.get_face(point);
        let next_face = self.get_face(next_point);

        if prev_face != next_face {
            let (next_face, wrapped_direction) = WRAPPINGS[prev_face.unwrap()][direction];

            next_direction = wrapped_direction;
            let (mut wx, mut wy) = (nx.rem_euclid(50), ny.rem_euclid(50));

            let pdir = DIRECTIONS[direction];
            let ndir = DIRECTIONS[next_direction];

            let mut sign_changed = (pdir.x + pdir.y) * (ndir.x + ndir.y) == -1;
            let axis_changed = pdir.x.abs() != ndir.x.abs();

            let size = 49;

            if axis_changed {
                (wx, wy) = (wy, wx);
                sign_changed = !sign_changed;
            }

            if sign_changed {
                (wx, wy) = (size - wx, size - wy);
            }

            next_point = FACE_TOP_LEFT[next_face]
                + match next_direction {
                    0 => (0, wy),
                    1 => (wx, 0),
                    2 => (size, wy),
                    3 => (wx, size),
                    _ => unreachable!(),
                }
                .into();
        }

        let next_tile = self.tiles[next_point.y as usize][next_point.x as usize];

        (next_point, next_tile, next_direction)
    }

    pub fn get_next_point(&self, point: Point, direction: usize) -> (Point, Tile) {
        let mut moving_point = point;
        let direction = DIRECTIONS[direction];

        loop {
            moving_point = moving_point + direction;

            if moving_point.x >= self.width() as i64 {
                moving_point = (moving_point.x - self.width() as i64, moving_point.y).into();
            }
            if moving_point.y >= self.height() as i64 {
                moving_point = (moving_point.x, moving_point.y - self.height() as i64).into();
            }
            if moving_point.x < 0 {
                moving_point = (self.width() as i64 + moving_point.x, moving_point.y).into();
            }
            if moving_point.y < 0 {
                moving_point = (moving_point.x, self.height() as i64 + moving_point.y).into();
            }

            let next_tile = self.tiles[moving_point.y as usize][moving_point.x as usize];
            match next_tile {
                Tile::Empty | Tile::Wall => return (moving_point, next_tile),
                Tile::NonExisting => {}
            }
        }
    }

    pub fn print_with_path(&self, path: &HashMap<Point, usize>) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(direction) = path.get(&(x, y).into()) {
                    let dir_char = match direction {
                        0 => '>',
                        1 => 'v',
                        2 => '<',
                        3 => '^',
                        _ => unreachable!(),
                    };

                    print!("{dir_char}");
                } else {
                    match cell {
                        Tile::Wall => print!("#"),
                        Tile::Empty => print!("."),
                        Tile::NonExisting => print!(" "),
                    };
                }
            }
            println!();
        }
    }
}

impl Display for WrappingMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for cell in row {
                match cell {
                    Tile::Wall => f.write_char('#')?,
                    Tile::Empty => f.write_char('.')?,
                    Tile::NonExisting => f.write_char(' ')?,
                };
            }
            f.write_char('\n')?;
        }
        Ok(())
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

fn process_input(input: &str) -> (WrappingMap, Vec<Move>) {
    let mut sections = input.split("\n\n");

    let map = WrappingMap::from_str(sections.next().unwrap());

    let moves_raw = sections.next().unwrap();
    let mut moves = vec![];

    let mut number = "".to_string();
    for character in moves_raw.as_bytes() {
        match character {
            b'0'..=b'9' => number.push(char::from_u32(*character as u32).unwrap()),
            b'R' => {
                if !number.is_empty() {
                    moves.push(Move::Forward(number.parse().unwrap()));
                    number.clear();
                }

                moves.push(Move::TurnClockwise)
            }
            b'L' => {
                if !number.is_empty() {
                    moves.push(Move::Forward(number.parse().unwrap()));
                    number.clear();
                }

                moves.push(Move::TurnCounterclockwise)
            }
            _ => {}
        }
    }

    if !number.is_empty() {
        moves.push(Move::Forward(number.parse().unwrap()));
        number.clear();
    }

    (map, moves)
}

fn solve_a(input: &str) -> i64 {
    let (map, moves) = process_input(input);

    let mut path = HashMap::new();

    let mut position = map.get_starting_position();
    let mut direction_index = 0;

    for movement in moves {
        match movement {
            Move::TurnClockwise => {
                direction_index = (direction_index + 1) % DIRECTIONS.len();
            }

            Move::TurnCounterclockwise => {
                direction_index = if direction_index == 0 {
                    DIRECTIONS.len() - 1
                } else {
                    direction_index - 1
                }
            }

            Move::Forward(amount) => {
                for _ in 0..amount {
                    let (next_position, next_tile) = map.get_next_point(position, direction_index);

                    if next_tile == Tile::Empty {
                        position = next_position;

                        path.insert(position, direction_index);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    (position.x + 1) * 4 + (position.y + 1) * 1000 + direction_index as i64
}

fn solve_b(input: &str) -> i64 {
    let (map, moves) = process_input(input);

    let mut path = HashMap::new();

    let mut position = map.get_starting_position();
    let mut direction_index = 0;

    for movement in moves {
        match movement {
            Move::TurnClockwise => {
                direction_index = (direction_index + 1) % DIRECTIONS.len();
            }

            Move::TurnCounterclockwise => {
                direction_index = if direction_index == 0 {
                    DIRECTIONS.len() - 1
                } else {
                    direction_index - 1
                }
            }

            Move::Forward(amount) => {
                for _ in 0..amount {
                    let (next_position, next_tile, next_direction) =
                        map.get_next_point_cube(position, direction_index);

                    if next_tile == Tile::Empty {
                        position = next_position;
                        direction_index = next_direction;

                        path.insert(position, direction_index);
                    } else if next_tile == Tile::NonExisting {
                        panic!("Invalid tile.");
                    } else {
                        break;
                    }
                }
            }
        }
    }

    (position.x + 1) * 4 + (position.y + 1) * 1000 + direction_index as i64
}
