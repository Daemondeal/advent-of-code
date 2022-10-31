fn main() {
    let input = include_str!("input.txt");

    println!("{}", solve_a(input));
    println!("{}", solve_b(input));    
}

fn solve_b(input: &str) -> i32 {
    let mut range = input.trim_end().split('-');
    let start: i32 = range.next().unwrap().parse().unwrap();
    let end: i32 = range.next().unwrap().parse().unwrap();
    let mut count = 0;

    println!("{}, {}", start, end);

    for n in start..end + 1 {
        if is_valid_b(n) {
            count += 1;
        }
    }
    
    count
}

fn is_valid_b(n: i32) -> bool {
    if !(100_000..999_999).contains(&n) {
        return false
    }

    let digits: Vec<char> = n.to_string().chars().collect();

    for i in 0..5 {
        if digits[i+1] < digits[i] {
            return false;
        }
    }

    let mut similar = 0;

    for i in 0..5 {

        if digits[i+1] == digits[i] {
            similar += 1;
        } else { 
            if similar == 1 {
                return true;
            }

            similar = 0;
        }
    }

    similar == 1
}

fn solve_a(input: &str) -> i32 {
    let mut range = input.trim_end().split('-');
    let start: i32 = range.next().unwrap().parse().unwrap();
    let end: i32 = range.next().unwrap().parse().unwrap();
    let mut count = 0;

    println!("{}, {}", start, end);

    for n in start..end + 1 {
        if is_valid(n) {
            count += 1;
        }
    }
    
    count
}

fn is_valid(n: i32) -> bool {
    if !(100_000..999_999).contains(&n) {
        return false
    }

    let digits: Vec<char> = n.to_string().chars().collect();

    for i in 0..5 {
        if digits[i+1] < digits[i] {
            return false;
        }
    }

    for i in 0..5 {
        if digits[i+1] == digits[i] {
            return true;
        }
    }

    false
}