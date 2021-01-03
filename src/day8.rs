use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        let pattern = Regex::new(r"^(acc|jmp|nop) (\+\d+|\-\d+)$").unwrap();
        let cap = pattern.captures(input).unwrap();
        let val = cap.get(2).unwrap().as_str().parse::<isize>().unwrap();
        match cap.get(1).unwrap().as_str() {
            "acc" => Self::Acc(val),
            "jmp" => Self::Jmp(val),
            "nop" => Self::Nop(val),
            _ => panic!("Invalid operation"),
        }
    }
}

type Program = Vec<Op>;

#[derive(Debug)]
enum ProgramError {
    InfiniteLoop,
    OutOfBounds,
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramError::InfiniteLoop => write!(f, "Program entered infinite loop."),
            ProgramError::OutOfBounds => write!(f, "Program jumped out of bounds."),
        }
    }
}
impl Error for ProgramError {}

struct VirtualMachine {
    ip: usize,
    acc: isize,
}

impl VirtualMachine {
    fn new() -> Self {
        Self { ip: 0, acc: 0 }
    }

    fn run(&mut self, prog: &Program) -> Result<isize, ProgramError> {
        let mut visited = HashSet::new();
        loop {
            if visited.contains(&self.ip) {
                return Err(ProgramError::InfiniteLoop);
            }
            visited.insert(self.ip);
            if self.ip == prog.len() {
                return Ok(self.acc);
            }
            match prog.get(self.ip) {
                Some(Op::Acc(v)) => {
                    self.acc += v;
                    self.ip += 1;
                }
                Some(Op::Jmp(v)) => {
                    let offset = usize::try_from(v.abs()).unwrap();
                    if *v > 0 {
                        self.ip += offset;
                    } else if offset < prog.len() {
                        self.ip -= offset;
                    } else {
                        return Err(ProgramError::OutOfBounds);
                    }
                }
                Some(Op::Nop(_)) => self.ip += 1,
                None => return Err(ProgramError::OutOfBounds),
            }
        }
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Program {
    input.lines().map(|x| Op::from(x)).collect()
}

#[aoc(day8, part1)]
fn run_once(prog: &Program) -> Result<isize, ProgramError> {
    let mut vm = VirtualMachine::new();
    let ret = vm.run(prog);
    if let Err(ProgramError::InfiniteLoop) = ret {
        Ok(vm.acc)
    } else {
        ret
    }
}

#[aoc(day8, part2)]
fn fix_prog(prog: &Program) -> Option<isize> {
    for (i, op) in prog.iter().enumerate() {
        let new_op = match op {
            Op::Acc(_) => continue,
            Op::Jmp(v) => Op::Nop(*v),
            Op::Nop(v) => Op::Jmp(*v),
        };
        let mut test_prog = prog.clone();
        test_prog[i] = new_op;
        let mut vm = VirtualMachine::new();
        if let Ok(v) = vm.run(&test_prog) {
            return Some(v);
        }
    }
    None
}
