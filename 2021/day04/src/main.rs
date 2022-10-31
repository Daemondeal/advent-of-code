use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    

    println!("A: {}", solve_a(input));
    println!("B: {}", solve_b(input));
}

#[derive(Debug)]
struct BingoCard {
    pub numbers: Vec<Vec<i32>>,
    pub marked: HashSet<i32>,
    pub id: i32
}

impl BingoCard {
    pub fn new(numbers: Vec<Vec<i32>>, id: i32) -> Self {


        Self { numbers, marked: HashSet::new(), id }
    }

    pub fn contains(&self, number: i32) -> bool {
        for row in self.numbers.iter() {
            if row.contains(&number) {
                return true;
            }
        }

        false
    }

    pub fn mark(&mut self, number: i32) {
        self.marked.insert(number);
    }

    pub fn points(&self) -> i32 {
        let mut points = 0;

        for row in self.numbers.iter() {
            for n in row.iter() {
                if !self.marked.contains(n) {
                    points += n;
                }
            }
        }

        points
    }

    pub fn has_won(&self) -> bool {
        // Rows
        for row in self.numbers.iter() {
            let mut flag = true;
            for n in row {
                if !self.marked.contains(&n) {
                    flag = false;
                    break;
                }
            }

            if flag { return true }
        }

        // Cols
        for i in 0..self.numbers[0].len() {
            let mut flag = true;
            for j in 0..self.numbers.len() {
                if !self.marked.contains(&self.numbers[j][i]) {
                    flag = false;
                    break;
                }
            }

            if flag { return true }
        }

        false
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoCard>) {
    let mut it = input.split("\n\n");
    let mut bingo_cards = vec![];

    let numbers = it.next()
        .unwrap()
        .split(",")
        .map(|x| x.parse())
        .flatten()
        .collect::<Vec<i32>>();


    let mut i = 0;

    for card in it {
        let bingo = card.split("\n")
            .map(|x| x.split(" ").map(|x| x.parse()).flatten().collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        bingo_cards.push(BingoCard::new(bingo, i));

        i += 1;
    }

    (numbers, bingo_cards)
    
}

fn solve_a(input: &str) -> i32 {
    let (numbers, mut cards) = parse_input(input);


    for n in numbers {
        for card in cards.iter_mut() {
            if card.contains(n) {
                card.mark(n);

                if card.has_won() {
                    return card.points() * n
                }
            }
        }
    }

    -1
}

fn solve_b(input: &str) -> i32 {
    let (numbers, mut cards) = parse_input(input);

    let mut won_cards = HashSet::new();
    let mut last_win_points = 0;


    for n in numbers {
        for card in cards.iter_mut() {
            if !won_cards.contains(&card.id) && card.contains(n) {
                card.mark(n);

                if card.has_won() {
                    last_win_points = card.points() * n;
                    won_cards.insert(card.id);
                }
            }
        }
    }

    last_win_points
}
