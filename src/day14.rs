use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Program {
    instructions: Vec<Instruction>,
}

// TODO: Read more about union types
enum Instruction {
    // We don't parse the mask because it gets handled differently in part1 vs part2
    Mask(String),
    Mem { addr: u64, value: u64 },
}

impl Program {
    pub fn run_v1(&self) -> HashMap<u64, u64> {
        let mut mem_map: HashMap<u64, u64> = HashMap::new();
        let mut cur_and_mask: u64 = 0;
        let mut cur_or_mask: u64 = 0;
        for instr in self.instructions.iter() {
            match instr {
                Instruction::Mem { addr, value } => {
                    mem_map.insert(*addr, value & cur_and_mask | cur_or_mask);
                }
                Instruction::Mask(mask_str) => {
                    lazy_static! {
                        static ref MASK_RE: Regex = Regex::new(r"[X01]").unwrap();
                    }
                    let masks =
                        MASK_RE.find_iter(mask_str).fold((0, 0), |(and, or), mat| {
                            match mat.as_str() {
                                "0" => ((and << 1) + 0, (or << 1) + 0),
                                "1" => ((and << 1) + 1, (or << 1) + 1),
                                "X" => ((and << 1) + 1, (or << 1) + 0),
                                _ => panic!(),
                            }
                        });
                    cur_and_mask = masks.0;
                    cur_or_mask = masks.1;
                }
            }
        }
        mem_map
    }

    pub fn run_v2(&self) -> HashMap<u64, u64> {
        let mut mem_map: HashMap<u64, u64> = HashMap::new();
        let mut cur_and_mask: u64 = 0;
        let mut cur_or_mask: u64 = 0;
        let mut cur_float_mask: Vec<u64> = vec![0];
        for instr in self.instructions.iter() {
            match instr {
                Instruction::Mem { addr, value } => {
                    for float in cur_float_mask.iter() {
                        mem_map.insert(*addr & cur_and_mask | cur_or_mask | float, *value);
                    }
                }
                Instruction::Mask(mask_str) => {
                    lazy_static! {
                        static ref MASK_RE: Regex = Regex::new(r"[X01]").unwrap();
                    }
                    let masks = MASK_RE.find_iter(mask_str).fold(
                        (0, 0, vec![0]),
                        |(and_mask, or_mask, float_mask), mat| match mat.as_str() {
                            "X" => (
                                (and_mask << 1) + 0,
                                (or_mask << 1) + 0,
                                float_mask
                                    .iter()
                                    .map(|f| f << 1)
                                    .chain(float_mask.iter().map(|f| (f << 1) + 1))
                                    .collect(),
                            ),
                            "1" => (
                                (and_mask << 1) + 1,
                                (or_mask << 1) + 1,
                                float_mask.iter().map(|f| f << 1).collect(),
                            ),
                            "0" => (
                                (and_mask << 1) + 1,
                                (or_mask << 1) + 0,
                                float_mask.iter().map(|f| f << 1).collect(),
                            ),
                            _ => panic!(),
                        },
                    );
                    cur_and_mask = masks.0;
                    cur_or_mask = masks.1;
                    cur_float_mask = masks.2;
                }
            }
        }
        mem_map
    }

    pub fn parse(source: &str) -> Program {
        lazy_static! {
            static ref INSTR_RE: Regex =
                Regex::new(r"(mem\[(\d+)] = (\d+))|(mask = ([01X]+))").unwrap();
        }
        Program {
            instructions: INSTR_RE
                .captures_iter(source)
                .map(|caps| {
                    if caps.get(1).is_some() {
                        Instruction::Mem {
                            addr: caps[2].parse::<u64>().unwrap(),
                            value: caps[3].parse::<u64>().unwrap(),
                        }
                    } else {
                        Instruction::Mask(caps[5].to_string())
                    }
                })
                .collect(),
        }
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let program = Program::parse(input);
    program.run_v1().values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let program = Program::parse(input);
    program.run_v2().values().sum()
}
