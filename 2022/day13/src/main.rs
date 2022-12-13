use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Num(i32),
    List(Vec<Item>),
}

struct ItemParser {
    line: Vec<u8>,
    position: usize,
}

impl ItemParser {
    fn new(line: &str) -> Self {
        Self {
            line: line.as_bytes().to_vec(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.line.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let element = self.line.get(self.position);
        self.position += 1;
        element.copied()
    }

    fn consume_token(&mut self) -> String {
        match self.peek() {
            None => "".to_string(),
            Some(b'[') | Some(b']') | Some(b',') => {
                String::from_utf8(vec![self.advance().unwrap()]).unwrap()
            }
            Some(_) => {
                let mut result = "".to_string();

                while let Some(num) = self.peek() {
                    if !(b'0'..=b'9').contains(&num) {
                        break;
                    }

                    let char = self.advance().unwrap();
                    result.push(char::from_u32(char as u32).unwrap());
                }

                result
            }
        }
    }

    fn parse(&mut self) -> Item {
        let tok = self.consume_token();
        match tok.as_str() {
            "[" => {
                let mut items = vec![];

                while self.peek() != Some(b']') {
                    items.push(self.parse());

                    if self.peek() == Some(b',') {
                        self.advance();
                    }
                }
                self.advance();

                Item::List(items)
            }
            num => Item::Num(
                num.parse()
                    .unwrap_or_else(|_| panic!("Invalid digit: {num}")),
            ),
        }
    }
}

impl Item {
    fn from_line(line: &str) -> Self {
        ItemParser::new(line).parse()
    }

    fn cmp(&self, other: &Item) -> Ordering {
        match check_order(self, other) {
            Decision::InOrder => Ordering::Less,
            Decision::OutOfOrder => Ordering::Greater,
            Decision::CantDecide => Ordering::Equal,
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

fn process_input(input: &str) -> Vec<(Item, Item)> {
    let mut pairs = vec![];

    for pair in input.split("\n\n") {
        let mut pair_lines = pair.split('\n');

        let Some(first) = pair_lines.next() else { break; };
        let Some(second) = pair_lines.next() else { break; };

        pairs.push((Item::from_line(first), Item::from_line(second)));
    }

    pairs
}

#[derive(PartialEq, Eq, Debug)]
enum Decision {
    InOrder,
    OutOfOrder,
    CantDecide,
}

fn check_order(left: &Item, right: &Item) -> Decision {
    match (left, right) {
        (Item::Num(l), Item::Num(r)) => match l.cmp(r) {
            Ordering::Less => Decision::InOrder,
            Ordering::Equal => Decision::CantDecide,
            Ordering::Greater => Decision::OutOfOrder,
        },
        (Item::List(lhs), Item::List(rhs)) => {
            for i in 0..lhs.len() {
                if i >= rhs.len() {
                    return Decision::OutOfOrder;
                }

                let order = check_order(&lhs[i], &rhs[i]);
                if order != Decision::CantDecide {
                    return order;
                }
            }
            if lhs.len() < rhs.len() {
                Decision::InOrder
            } else {
                Decision::CantDecide
            }
        }
        (Item::Num(_), Item::List(_)) => check_order(&Item::List(vec![left.clone()]), right),
        (Item::List(_), Item::Num(_)) => check_order(left, &Item::List(vec![right.clone()])),
    }
}

fn solve_a(input: &str) -> i32 {
    let pairs = process_input(input);

    let mut res = 0;

    for (i, (left, right)) in pairs.iter().enumerate() {
        let order = check_order(left, right);
        if order == Decision::InOrder {
            res += i as i32 + 1;
        }
    }

    res
}

fn solve_b(input: &str) -> usize {
    let pairs = process_input(input);

    let mut signals = vec![];

    for (left, right) in pairs {
        signals.push(left);
        signals.push(right);
    }

    let first = Item::List(vec![Item::List(vec![Item::Num(2)])]);
    let second = Item::List(vec![Item::List(vec![Item::Num(6)])]);

    signals.push(first.clone());
    signals.push(second.clone());

    signals.sort_by(|x, y| x.cmp(y));

    let mut first_index = 0;
    let mut second_index = 0;

    for (i, signal) in signals.iter().enumerate() {
        if signal == &first {
            first_index = i + 1;
        } else if signal == &second {
            second_index = i + 1;
        }
    }

    first_index * second_index
}
