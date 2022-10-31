#![allow(dead_code)]
#![allow(clippy::println_empty_string)]

use std::vec::Vec;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Params(Vec<ParamMode>);

#[derive(PartialEq, Debug, Clone)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative
}

impl Params {
    fn from_instruction(instruction: i64) -> Self {
        let mut params = vec![];
        let mut buffer = instruction / 100;

        while buffer > 0 {
            params.push(Self::to_param((buffer % 10) as u8));

            buffer /= 10;
        }

        Self(params)
    }

    fn to_param(val: u8) -> ParamMode {
        match val {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            x => { panic!("Invalid parameter {}", x) }
        }
    }

    fn get(&self, i: usize) -> ParamMode {
        if let Some(res) = self.0.get(i) {
            res.clone()
        } else {
            ParamMode::Position
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
    AddToRelativeBase,
    
    Halt,
}

impl Opcode {

    pub fn try_from(v: i64) -> Result<Self, ()> {
        match v {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            9 => Ok(Opcode::AddToRelativeBase),
            99 => Ok(Opcode::Halt),
            _ => Err(())
        }
    }
}

pub struct IntcodeMachine {
    pub state: Vec<i64>,
    pub ip: usize,
    pub debug: bool,
    pub relative_base: i64,
    pub extra_memory: HashMap<usize, i64>,
}

impl Default for IntcodeMachine {
    fn default() -> Self {
        IntcodeMachine {
            state: vec![],
            ip: 0,
            debug: false,
            relative_base: 0,
            extra_memory: HashMap::new(),
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
            debug: self.debug,
            relative_base: self.relative_base,
            extra_memory: self.extra_memory.clone(),
        }
    }

    fn get(&self, address: usize) -> i64 {
        if address < self.state.len() {
            self.state[address]
        } else if let Some(value) = self.extra_memory.get(&address) {
            *value
        } else {
            0
        }
    }

    fn set(&mut self, address: usize, value: i64) {
        if address < self.state.len() {
            self.state[address] = value;
        } else {
            self.extra_memory.insert(address, value);
        }
    }

    fn get_by_mode(&self, address: usize, mode: ParamMode) -> i64 {
        match mode {
            ParamMode::Immediate => self.get(address),
            ParamMode::Position => self.get(self.get(address) as usize),
            ParamMode::Relative => self.get(((self.relative_base as i64) + self.get(address)) as usize)
        }
    }

    fn get_target_by_mode(&self, address: usize, mode: ParamMode) -> usize {
        match mode {
            ParamMode::Immediate => panic!("Cannot target something in immediate mode."),
            ParamMode::Position => self.get(address) as usize,
            ParamMode::Relative => (self.relative_base + self.get(address)) as usize,
        }
    }

    pub fn run_manual(&mut self) {
        self.run(
            || { // Input
                print!("Input: ");

                io::stdout().flush().unwrap();
                (read_input(), false)
            },
            |x| { // Output
                println!("Output: {}", x);
            }
        )
    }

    pub fn run_until_input(&mut self) -> (bool, Vec<i64>) {
        let mut outputs = vec![];
        let mut got_to_input = false;

        self.run(
            || { // Input
                got_to_input = true;
                (0, true)
            },
            |x| { // Outputs
                outputs.push(x);
            }
        );

        (!got_to_input, outputs)
    }

    pub fn restore_with_input(&mut self, input: i64) -> (bool, Vec<i64>) {
        let mut outputs = vec![];
        let mut given = false;
        let mut got_to_input = false;

        self.run(
            || { // Input
                if !given {
                    given = true;

                    return (input, false);
                }

                got_to_input = true;
                (0, true)
            },
            |x| { // Outputs
                outputs.push(x);
            }
        );

        (!got_to_input, outputs)
    }

    pub fn run_collecting_outputs(&mut self, inputs: &[i64]) -> Vec<i64> {
        let mut outputs = vec![];
        let mut inputs_pointer = 0;
        
        self.run(
            || { // Input
                let input = inputs[inputs_pointer];
                inputs_pointer += 1;
                if inputs_pointer > inputs.len() {
                    panic!("Not enough inputs.\n");
                }


                (input, false)
            },
            |x| { // Output
                outputs.push(x);
            }
        );

        outputs
    }

    fn run(&mut self, mut input_func: impl FnMut() -> (i64, bool), mut output_func: impl FnMut(i64)) {
        'program_loop: loop {
            let opcode = self.get(self.ip) % 100;
            
            let params = Params::from_instruction(self.get(self.ip));

            if self.debug {
                if let Ok(op) = Opcode::try_from(opcode) { 
                    println!(
                        "Executing {:?} (ip: {}), params: {:?} next: {} {} {}", 
                        op, 
                        self.ip,
                        params,
                        self.get(self.ip + 1),
                        self.get(self.ip + 2),
                        self.get(self.ip + 3),
                    );
                }
            }


            match Opcode::try_from(opcode) {
                Ok(Opcode::Add) => {
                    // Add
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));
                    let target = self.get_target_by_mode(self.ip + 3, params.get(2));

                    self.set(target, first + second);

                    self.ip += 4;
                }
                Ok(Opcode::Multiply) => {
                    // Multiply
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));
                    let target = self.get_target_by_mode(self.ip + 3, params.get(2));

                    self.set(target, first * second);

                    self.ip += 4;
                }
                Ok(Opcode::Input) => {
                    // Input
                    let target = self.get_target_by_mode(self.ip + 1, params.get(0));
                    
                    let (input, should_wait) = input_func();
                    if should_wait {
                        break 'program_loop;
                    }

                    self.set(target, input);

                    self.ip += 2;

                }
                Ok(Opcode::Output) => {
                    // Output
                    let num = self.get_by_mode(self.ip + 1, params.get(0));

                    output_func(num);

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
                    let target = self.get_target_by_mode(self.ip + 3, params.get(2));
                    
                    if first < second {
                        self.set(target, 1);
                    }
                    else {
                        self.set(target, 0);
                    }

                    self.ip += 4;
                }
                Ok(Opcode::Equals) => {
                    // Equals
                    let first = self.get_by_mode(self.ip + 1, params.get(0));
                    let second = self.get_by_mode(self.ip + 2, params.get(1));
                    let target = self.get_target_by_mode(self.ip + 3, params.get(2));
                    
                    
                    if first == second {
                        self.set(target, 1);
                    } else {
                        self.set(target, 0);
                    }

                    self.ip += 4;
                }
                Ok(Opcode::AddToRelativeBase) => {
                    let val = self.get_by_mode(self.ip + 1, params.get(0));
                    
                    self.relative_base += val;

                    self.ip += 2;
                }
                Ok(Opcode::Halt) => {
                    // Halt

                    break 'program_loop;
                }

                Err(()) => {
                    panic!("ERROR: Invalid opcode at position {}: {}.", self.ip, self.get(self.ip) % 100);
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

fn read_input() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
