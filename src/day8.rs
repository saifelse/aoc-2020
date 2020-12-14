use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

struct Program {
    acc: i64,
    eip: usize,
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
enum Operator {
    ACC,
    NOP,
    JMP,
}

#[derive(Clone)]
struct Instruction {
    operator: Operator,
    argument: i64,
}


impl Program {
    pub fn step(&mut self) {
        let instr = &self.instructions[self.eip];
        match instr.operator {
            Operator::ACC => {
                self.acc += instr.argument;
                self.eip += 1;
            },
            Operator::NOP => {
                self.eip += 1;
            },
            Operator::JMP => {
                self.eip += instr.argument as usize;
            },
        };
    }

    pub fn run(&mut self) -> std::result::Result<i64, i64> {
        let mut seen_pointers: HashSet<usize> = HashSet::new();
        while !seen_pointers.contains(&self.eip) {
            seen_pointers.insert(self.eip);
            self.step();
            if self.eip == self.instructions.len() {
                return Ok(self.acc);
            }
        }
        Err(self.acc)
    }
    
    pub fn fork(
        &self,
        idx: usize,
        repl_instruction: Instruction,
    ) -> Program {
        Program {
            acc: 0,
            eip: 0,
            instructions: self.instructions[0..idx]
                .iter()
                .cloned()
                .chain(std::iter::once(repl_instruction))
                .chain(self.instructions[idx + 1..].iter().cloned())
                .collect(),
        }
    }

    pub fn parse(source: &str) -> Program {
        lazy_static! {
            static ref INSTR_RE: Regex = Regex::new(r"(acc|jmp|nop) ([+-]\d+)").unwrap();
        }
        Program {
            acc: 0,
            eip: 0,
            instructions: INSTR_RE
                .captures_iter(source)
                .map(|caps| {
                    let operator = match &caps[1] {
                        "acc" => Operator::ACC,
                        "jmp" => Operator::JMP,
                        "nop" => Operator::NOP,
                        _ => panic!(),
                    };
                    let argument = *&caps[2].parse::<i64>().unwrap();
                    Instruction { operator, argument }
                })
                .collect(),
        }
    }
}


#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let program = &mut Program::parse(input);
    match program.run() {
        Err(acc) => acc,
        _ => panic!("Expected program to error"),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let program = Program::parse(input);
    for (idx, instruction) in program.instructions.iter().enumerate() {
        let repl_program = &mut program.fork(
            idx,
            Instruction {
                operator: match instruction.operator {
                    Operator::ACC => Operator::ACC,
                    Operator::JMP => Operator::NOP,
                    Operator::NOP => Operator::JMP,
                },
                argument: instruction.argument,
            },
        );
        if let Ok(acc) = repl_program.run() {
            return acc;
        }
    }
    panic!("Expected one of the program variants to exit successfully.");
}
