#![allow(dead_code)]
#![allow(clippy::println_empty_string)]

use std::vec::Vec;

pub struct IntcodeMachine {
    pub state: Vec<i32>,
    pub ip: usize,
}

impl Default for IntcodeMachine {
    fn default() -> Self {
        IntcodeMachine {
            state: vec![],
            ip: 0,
        }
    }
}

impl IntcodeMachine {
    pub fn parse(source: &str) -> Self {
        let state = source.split(',')
            .map(|x| x.trim_end().parse())
            .flatten()
            .collect();
        
        IntcodeMachine {
            state,
            ..IntcodeMachine::default()
        }
    }

    pub fn clone(&self) -> Self {
        IntcodeMachine {
            state: self.state.clone(),
            ip: self.ip,
        }
    }

    pub fn run(&mut self) {
        'program_loop: loop {
            match self.state[self.ip] {
                1 => {
                    // Add
                    let first = self.state[self.ip + 1] as usize;
                    let second = self.state[self.ip + 2] as usize;
                    let target = self.state[self.ip + 3] as usize;

                    self.state[target] = self.state[first] + self.state[second];

                    self.ip += 4;
                }
                2 => {
                    // Multiply
                    let first = self.state[self.ip + 1] as usize;
                    let second = self.state[self.ip + 2] as usize;
                    let target = self.state[self.ip + 3] as usize;

                    self.state[target] = self.state[first] * self.state[second];

                    self.ip += 4;
                }
                99 => {
                    // Halt

                    break 'program_loop;
                }

                x => {
                    panic!("ERROR: Invalid opcode at position {}: {}.", self.ip, x);
                }
            }
        }
    }

    pub fn print_state(&self) {
        println!("ip: {}", self.ip);
        println!("state: ");

        for chunk in self.state.chunks(4) {
            chunk.iter().for_each(|x| print!("{} ", x));
            println!("");
        }
    }
}