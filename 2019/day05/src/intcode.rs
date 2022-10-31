#![allow(dead_code)]
#![allow(clippy::println_empty_string)]

use std::vec::Vec;
use std::io::{self, Write};

pub struct Params(Vec<bool>);

#[derive(PartialEq, Debug)]
pub enum ParamMode {
    Position,
    Immediate
}

impl Params {
    fn from_instruction(instruction: i32) -> Self {
        let mut params = vec![];
        let mut buffer = instruction / 100;

        while buffer > 0 {
            params.push(buffer % 10 > 0);

            buffer /= 10;
        }

        Self(params)
    }

    fn get(&self, i: usize) -> ParamMode {
        match self.0.get(i) {
            Some(true) => ParamMode::Immediate,
            Some(false) => ParamMode::Position,
            None => ParamMode::Position,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    
    Halt,
}

impl Opcode {

    pub fn try_from(v: i32) -> Result<Self, ()> {
        match v {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            99 => Ok(Opcode::Halt),
            _ => Err(())
        }
    }
}

pub struct IntcodeMachine {
    pub state: Vec<i32>,
    pub ip: usize,
    pub debug: bool,
}

impl Default for IntcodeMachine {
    fn default() -> Self {
        IntcodeMachine {
            state: vec![],
            ip: 0,
            debug: false,
        }
    }
}

impl IntcodeMachine {
    pub fn parse(source: &str) -> Self {
        let state = source.split(',')
            .map(|x| x.trim_start().trim_end().parse())
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
            debug: self.debug
        }
    }

    fn get_by_mode(&self, address: usize, mode: ParamMode) -> i32 {
        match mode {
            ParamMode::Immediate => self.state[address],
            ParamMode::Position => self.state[self.state[address] as usize],
        }
    }

    pub fn run(&mut self) {
        'program_loop: loop {
            let opcode = self.state[self.ip] % 100;
            
            if self.debug {
                if let Ok(op) = Opcode::try_from(opcode) { 
                    println!("Executing {:?} (ip: {})...", op, self.ip);
                }
            }

            let params = Params::from_instruction(self.state[self.ip]);

            match Opcode::try_from(opcode) {
                Ok(Opcode::Add) => {
                    // Add
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));

                    if params.get(2) != ParamMode::Position {
                        panic!("Invalid paramter 2 for instruction of type Add: {:?}", params.get(2));
                    }
                    let target = self.state[self.ip + 3] as usize;

                    self.state[target] = first + second;

                    self.ip += 4;
                }
                Ok(Opcode::Multiply) => {
                    // Multiply
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));

                    if params.get(2) != ParamMode::Position {
                        panic!("Invalid paramter 2 for instruction of type Multiply: {:?}", params.get(2));
                    }
                    let target = self.state[self.ip + 3] as usize;

                    self.state[target] = first * second;

                    self.ip += 4;
                }
                Ok(Opcode::Input) => {
                    // Input
                    if params.get(0) != ParamMode::Position {
                        panic!("Invalid paramter 0 for instruction of type Input: {:?}", params.get(0));
                    }
                    let pos = self.state[self.ip + 1] as usize;
                    
                    print!("Input: ");

                    io::stdout().flush().unwrap();
                    let input = read_input();
                    self.state[pos] = input;

                    self.ip += 2;
                }
                Ok(Opcode::Output) => {
                    // Output
                    let num = self.get_by_mode(self.ip + 1, params.get(0));

                    println!("Output: {}", num);

                    self.ip += 2;
                }
                Ok(Opcode::JumpIfTrue) => {
                    // Jump if true
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));

                    if first != 0 {
                        self.ip = second as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Ok(Opcode::JumpIfFalse) => {
                    // Jump if false
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));

                    if first == 0 {
                        self.ip = second as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Ok(Opcode::LessThan) => {
                    // Less than
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));
                    
                    if params.get(2) != ParamMode::Position {
                        panic!("Invalid paramter 2 for instruction of type Less Than: {:?}", params.get(2));
                    }
                    
                    let pos = self.state[self.ip + 3] as usize;
                    
                    if first < second {
                        self.state[pos] = 1;
                    }
                    else {
                        self.state[pos] = 0;
                    }

                    self.ip += 4;
                }
                Ok(Opcode::Equals) => {
                    // Equals
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));
                    
                    if params.get(2) != ParamMode::Position {
                        panic!("Invalid paramter 2 for instruction of type Equals: {:?}", params.get(2));
                    }
                    
                    let pos = self.state[self.ip + 3] as usize;
                    
                    if first == second {
                        self.state[pos] = 1;
                    } else {
                        self.state[pos] = 0;
                    }

                    self.ip += 4;
                }
                Ok(Opcode::Halt) => {
                    // Halt

                    break 'program_loop;
                }

                Err(()) => {
                    panic!("ERROR: Invalid opcode at position {}: {}.", self.ip, self.state[self.ip] % 100);
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

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}